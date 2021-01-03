import jetbrains.buildServer.configs.kotlin.v2019_2.*
import jetbrains.buildServer.configs.kotlin.v2019_2.buildFeatures.dockerSupport
import jetbrains.buildServer.configs.kotlin.v2019_2.buildSteps.dockerCommand
import jetbrains.buildServer.configs.kotlin.v2019_2.buildSteps.exec
import jetbrains.buildServer.configs.kotlin.v2019_2.triggers.ScheduleTrigger
import jetbrains.buildServer.configs.kotlin.v2019_2.triggers.schedule
import jetbrains.buildServer.configs.kotlin.v2019_2.triggers.vcs

/*
The settings script is an entry point for defining a TeamCity
project hierarchy. The script should contain a single call to the
project() function with a Project instance or an init function as
an argument.

VcsRoots, BuildTypes, Templates, and subprojects can be
registered inside the project using the vcsRoot(), buildType(),
template(), and subProject() methods respectively.

To debug settings scripts in command-line, run the

    mvnDebug org.jetbrains.teamcity:teamcity-configs-maven-plugin:generate

command and attach your debugger to the port 8000.

To debug in IntelliJ Idea, open the 'Maven Projects' tool window (View
-> Tool Windows -> Maven Projects), find the generate task node
(Plugins -> teamcity-configs -> teamcity-configs:generate), the
'Debug' option is available in the context menu for the task.
*/

version = "2020.1"

project {
    description = "rust messaging application"

    buildType(musl)
    buildType(primary)
}

object musl : BuildType({
    name = "musl"
    description = "Build project for use with musl"

    allowExternalStatus = true
    artifactRules = """
        reports => reports.zip
        coverage => coverage.zip
    """.trimIndent()

    params {
        param("env.RUSTUP_TOOLCHAIN", "nightly-x86_64-unknown-linux-gnu")
    }

    vcs {
        root(DslContext.settingsRoot)
    }

    steps {
        exec {
            name = "Install musl toolchain"
            path = "cargo"
            arguments = "make install-rustup-musl-linux"
        }
        exec {
            name = "Set toolchain"
            path = "cargo"
            arguments = "make nightly-musl"
            param("script.content", "cargo make format-check")
        }
        exec {
            name = "Update toolchain"
            path = "cargo"
            arguments = "make update"
        }
        exec {
            name = "Check formatting"
            path = "cargo"
            arguments = "make format-check"
            param("script.content", "cargo make format-check")
        }
        exec {
            name = "Clippy!"
            enabled = false
            path = "cargo"
            arguments = "make clippy"
            param("script.content", "cargo make clippy")
        }
        exec {
            name = "Test"
            path = "cargo"
            arguments = "make test"
            param("script.content", "cargo make docker-test")
        }
        exec {
            name = "Convert test output to junit"
            path = "cargo"
            arguments = "make junit"
            param("script.content", "cargo make junit")
        }
        exec {
            name = "Convert junit to html"
            path = "cargo"
            arguments = "make junit2html"
            param("script.content", "cargo make junit2html")
        }
        exec {
            name = "Grcov"
            path = "cargo"
            arguments = "make grcov"
            param("script.content", "cargo make coverage-report")
        }
        exec {
            name = "Coverage report"
            path = "cargo"
            arguments = "make coverage"
            param("script.content", "cargo make coverage-report")
        }
        exec {
            name = "Change toolchain"
            path = "cargo"
            arguments = "make nightly-linux-gnu"
        }
    }

    triggers {
        vcs {
            branchFilter = ""
        }
        schedule {
            schedulingPolicy = weekly {
                dayOfWeek = ScheduleTrigger.DAY.Friday
                hour = 8
            }
            branchFilter = ""
            triggerBuild = always()
            withPendingChangesOnly = false
            enforceCleanCheckout = true
        }
    }

    features {
        dockerSupport {
            cleanupPushedImages = true
            loginToRegistry = on {
                dockerRegistryId = "PROJECT_EXT_5"
            }
        }
        feature {
            type = "xml-report-plugin"
            param("xmlReportParsing.reportType", "junit")
            param("xmlReportParsing.reportDirs", "reports/test_results.xml")
        }
    }

    requirements {
        exists("Cargo")
    }
})

object primary : BuildType({
    name = "primary"
    description = "Build project for use in distroless container"

    allowExternalStatus = true
    artifactRules = """
        reports => reports.zip
        coverage => coverage.zip
    """.trimIndent()

    params {
        param("env.RUSTUP_TOOLCHAIN", "nightly-x86_64-unknown-linux-gnu")
    }

    vcs {
        root(DslContext.settingsRoot)
    }

    steps {
        exec {
            name = "Install toolchain"
            path = "cargo make"
            arguments = "install-rustup-linux-gnu"
        }
        exec {
            name = "Set target toolchain"
            path = "cargo"
            arguments = "make nightly-linux-gnu"
        }
        exec {
            name = "Update toolchain"
            path = "cargo"
            arguments = "make update"
        }
        exec {
            name = "Check formatting"
            path = "cargo"
            arguments = "make format-check"
            param("script.content", "cargo make format-check")
        }
        exec {
            name = "Clippy!"
            enabled = false
            path = "cargo"
            arguments = "make clippy"
            param("script.content", "cargo make clippy")
        }
        exec {
            name = "Test"
            path = "cargo"
            arguments = "make test"
            param("script.content", "cargo make docker-test")
        }
        exec {
            name = "Convert test output to junit"
            path = "cargo"
            arguments = "make junit"
            param("script.content", "cargo make junit")
        }
        exec {
            name = "Convert junit to html"
            path = "cargo"
            arguments = "make junit2html"
            param("script.content", "cargo make junit2html")
        }
        exec {
            name = "Grcov"
            path = "cargo"
            arguments = "make grcov"
            param("script.content", "cargo make coverage-report")
        }
        exec {
            name = "Coverage report"
            path = "cargo"
            arguments = "make coverage"
            param("script.content", "cargo make coverage-report")
        }
        exec {
            name = "Docker build"
            workingDir = "%system.teamcity.build.checkoutDir%"
            path = "docker"
            arguments = "build -t n0mn0m/artemis:latest -t n0mn0m/artemis:%build.vcs.number% -f ./docker/distroless.Dockerfile ."
        }
        dockerCommand {
            name = "Publish image"
            commandType = push {
                namesAndTags = """
                    n0mn0m/artemis:latest
                    n0mn0m/artemis:%build.vcs.number%
                """.trimIndent()
            }
        }
    }

    triggers {
        vcs {
            branchFilter = ""
        }
        schedule {
            schedulingPolicy = weekly {
                dayOfWeek = ScheduleTrigger.DAY.Friday
                hour = 8
            }
            branchFilter = ""
            triggerBuild = always()
            withPendingChangesOnly = false
            enforceCleanCheckout = true
        }
    }

    features {
        dockerSupport {
            cleanupPushedImages = true
            loginToRegistry = on {
                dockerRegistryId = "PROJECT_EXT_5"
            }
        }
        feature {
            type = "xml-report-plugin"
            param("xmlReportParsing.reportType", "junit")
            param("xmlReportParsing.reportDirs", "reports/test_results.xml")
        }
    }

    requirements {
        exists("Cargo")
    }
})
