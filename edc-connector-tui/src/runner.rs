use std::{collections::VecDeque, time::Duration};

use crossterm::event;
use ratatui::{backend::Backend, Terminal};

use crate::components::{Action, ActionHandler, Component, ComponentEvent};

pub struct Runner<C: Component> {
    tick_rate: Duration,
    component: C,
}

impl<C: Component + ActionHandler<Msg = <C as Component>::Msg> + Send> Runner<C> {
    pub fn new(tick_rate: Duration, component: C) -> Self {
        Self {
            tick_rate,
            component,
        }
    }

    pub async fn run(&mut self, mut terminal: Terminal<impl Backend>) -> anyhow::Result<()> {
        terminal.clear()?;

        let mut should_quit = false;
        loop {
            if should_quit {
                break;
            }
            terminal.draw(|frame| self.component.view(frame, frame.size()))?;

            if event::poll(self.tick_rate)? {
                let evt = event::read()?;
                let mut msgs = self
                    .component
                    .handle_event(ComponentEvent::Event(evt))?
                    .into_iter()
                    .collect::<VecDeque<_>>();

                while let Some(msg) = msgs.pop_front() {
                    let actions = {
                        let ret = self.component.update(msg).await?;

                        for m in ret.msgs {
                            msgs.push_back(m);
                        }

                        for c in ret.cmds {
                            for m in c.await.unwrap() {
                                msgs.push_back(m);
                            }
                        }

                        ret.actions
                    };

                    for a in actions {
                        should_quit = should_quit || matches!(a, Action::Quit);
                        for m in self.component.handle_action(a)? {
                            msgs.push_back(m)
                        }
                    }
                }
            };
        }

        Ok(())
    }
}
