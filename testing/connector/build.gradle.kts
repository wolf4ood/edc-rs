plugins {
    `java-library`
    id("application")
    alias(libs.plugins.shadow)
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

    implementation(libs.edc.data.plane.control.api)
    implementation(libs.edc.data.plane.public.api.v2)
    implementation(libs.edc.data.plane.signaling.api)
    implementation(libs.edc.data.plane.core)
    implementation(libs.edc.data.plane.http)
}

application {
    mainClass.set("org.eclipse.edc.boot.system.runtime.BaseRuntime")
}

tasks.withType<com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar> {
    dependsOn("distTar", "distZip")
    mergeServiceFiles()
    archiveFileName.set("connector.jar")
}
