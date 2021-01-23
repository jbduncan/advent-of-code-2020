import com.github.benmanes.gradle.versions.updates.DependencyUpdatesTask
import com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar
import org.jetbrains.kotlin.gradle.dsl.KotlinJvmCompile

buildscript {
    dependencyLocking {
        lockAllConfigurations()
    }
}

plugins {
    id("org.jetbrains.kotlin.jvm") version "1.4.21-2"
    id("com.github.ben-manes.versions") version "0.36.0"
    id("com.diffplug.spotless") version "5.9.0"
    id("com.github.johnrengelman.shadow") version "6.1.0"
}

group = "org.jbduncan.adventofcode2020.day7"
version = file("src/main/resources/version.txt").readText().trim()

sourceSets {
    create("intTest") {
        compileClasspath += sourceSets.main.get().output
        runtimeClasspath += sourceSets.main.get().output
    }
}

val intTestImplementation: Configuration by configurations.getting {
    extendsFrom(configurations.implementation.get())
}

configurations["intTestRuntimeOnly"].extendsFrom(configurations.runtimeOnly.get())

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
    implementation("com.google.guava:guava:30.1-jre")
    implementation("info.picocli:picocli:4.+")
    implementation("org.jgrapht:jgrapht-guava:1.+")
    implementation("org.jgrapht:jgrapht-core:1.+")

    testImplementation(platform("org.junit:junit-bom:5.+"))
    testImplementation("org.junit.jupiter:junit-jupiter") {
        because("it will allow JUnit 5 to be used")
    }
    testImplementation("com.google.truth:truth:1.+")

    intTestImplementation(platform("org.junit:junit-bom:5.+"))
    intTestImplementation("org.junit.jupiter:junit-jupiter") {
        because("it will allow JUnit 5 to be used")
    }
    intTestImplementation("com.google.truth:truth:1.+")
}

dependencyLocking {
    lockAllConfigurations()
}

tasks.register("updateLockfiles") {
    doFirst {
        require(gradle.startParameter.isWriteDependencyLocks) {
            "Run this task as './gradlew updateLockfiles --write-locks'"
        }
    }
    doLast {
        configurations.filter { it.isCanBeResolved }.forEach { it.resolve() }
    }
}

tasks.withType<Test> {
    useJUnitPlatform() // needed for JUnit 5
}

val integrationTest = task<Test>("integrationTest") {
    description = "Runs integration tests."
    group = "verification"

    testClassesDirs = sourceSets["intTest"].output.classesDirs
    classpath = sourceSets["intTest"].runtimeClasspath
    shouldRunAfter("test")
    dependsOn("shadowJar")
}

tasks.check { dependsOn(integrationTest) }

tasks.withType<ShadowJar> {
    manifest {
        attributes("Main-Class" to "org.jbduncan.adventofcode2020.day7.AppKt")
    }
}

val jvmVersion = 11

val compiler = javaToolchains.compilerFor {
    languageVersion.set(JavaLanguageVersion.of(jvmVersion))
}

tasks.withType<KotlinJvmCompile> {
    kotlinOptions {
        jdkHome = compiler.get().metadata.installationPath.asFile.absolutePath
        jvmTarget = "$jvmVersion"
    }
}

tasks.withType<AbstractArchiveTask> {
    isPreserveFileTimestamps = false
    isReproducibleFileOrder = true
}

tasks.named<DependencyUpdatesTask>("dependencyUpdates") {
    val regex = "^[0-9,.v-]+(-r)?$".toRegex()

    fun isNonStable(version: String): Boolean {
        val stableKeyword = listOf("RELEASE", "FINAL", "GA").any { version.toUpperCase().contains(it) }
        val isStable = stableKeyword || regex.matches(version)
        return isStable.not()
    }

    rejectVersionIf {
        isNonStable(candidate.version) && !isNonStable(currentVersion)
    }
}

spotless {
    kotlin {
        ktfmt("0.19")
    }
}
