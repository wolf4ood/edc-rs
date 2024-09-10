import com.bmuschko.gradle.docker.tasks.image.DockerBuildImage
import com.github.jengelman.gradle.plugins.shadow.ShadowJavaPlugin

plugins {
    `java-library`
    id("application")
    alias(libs.plugins.shadow)
    alias(libs.plugins.docker)
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(libs.edc.runtime.metamodel)
    implementation(libs.edc.control.plane.api.client)
    implementation(libs.edc.control.plane.core)
    implementation(libs.edc.control.plane.edr.receiver)
    implementation(libs.edc.dsp)
    implementation(libs.edc.core.jersey)
    implementation(libs.edc.core.jetty)
    implementation(libs.edc.configuration.filesystem)
    implementation(libs.edc.iam.mock)
    implementation(libs.edc.management.api)
    implementation(libs.edc.cache.api)
    implementation(libs.edc.edr.store.core)
    implementation(libs.edc.api.observability)
    implementation(libs.edc.transfer.signaling)
    implementation(libs.edc.validator.data.address.http.data)

    implementation(libs.edc.data.plane.selector.api)
    implementation(libs.edc.data.plane.selector.core)

    implementation(libs.edc.control.api.configuration)
    implementation(libs.edc.data.plane.public.api.v2)
    implementation(libs.edc.data.plane.signaling.api)
    implementation(libs.edc.data.plane.self.registration)
    implementation(libs.edc.data.plane.core)
    implementation(libs.edc.data.plane.http)
    implementation(libs.edc.data.plane.iam)
}

application {
    mainClass.set("org.eclipse.edc.boot.system.runtime.BaseRuntime")
}

tasks.withType<com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar> {
    dependsOn("distTar", "distZip")
    mergeServiceFiles()
    archiveFileName.set("connector.jar")
}

//actually apply the plugin to the (sub-)project
apply(plugin = "com.bmuschko.docker-remote-api")
// configure the "dockerize" task
val dockerTask: DockerBuildImage = tasks.create("dockerize", DockerBuildImage::class) {
    val dockerContextDir = project.projectDir
    dockerFile.set(file("$dockerContextDir/src/main/docker/Dockerfile"))
    images.add("${project.name}:${project.version}")
    images.add("${project.name}:latest")
    // specify platform with the -Dplatform flag:
    if (System.getProperty("platform") != null)
        platform.set(System.getProperty("platform"))
    buildArgs.put("JAR", "build/libs/${project.name}.jar")
    inputDir.set(file(dockerContextDir))
}
// make sure  always runs after "dockerize" and after "copyOtel"
dockerTask.dependsOn(tasks.named(ShadowJavaPlugin.SHADOW_JAR_TASK_NAME))
