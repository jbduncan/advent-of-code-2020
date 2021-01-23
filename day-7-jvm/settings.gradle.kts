rootProject.name = "day-7-jvm"

enableFeaturePreview("ONE_LOCKFILE_PER_PROJECT")

buildscript {
    dependencyLocking {
        lockAllConfigurations()
    }
}