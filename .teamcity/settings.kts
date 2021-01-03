import jetbrains.buildServer.configs.kotlin.v2019_2.*
import jetbrains.buildServer.configs.kotlin.v2019_2.buildFeatures.dockerSupport
import jetbrains.buildServer.configs.kotlin.v2019_2.buildSteps.PowerShellStep
import jetbrains.buildServer.configs.kotlin.v2019_2.buildSteps.dockerCommand
import jetbrains.buildServer.configs.kotlin.v2019_2.buildSteps.exec
import jetbrains.buildServer.configs.kotlin.v2019_2.buildSteps.powerShell
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
        powerShell {
            name = "Install toolchain"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command install-rustup-linux-musl")
        }
        powerShell {
            name = "Set toolchain"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command nightly-linux-musl")
        }
        powerShell {
            name = "Update toolchain"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command update")
        }
        powerShell {
            name = "Format check"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command format-check")
        }
        powerShell {
            name = "Clippy!"
            enabled = false
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command clippy")
        }
        powerShell {
            name = "Test"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command test")
        }
        powerShell {
            name = "Convert test output to junit"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command junit")
        }
        powerShell {
            name = "Convert junit to html"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command junit2html")
        }
        powerShell {
            name = "Grcov"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command grcov")
        }
        powerShell {
            name = "Generate coverage report"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command coverage")
        }
        powerShell {
            name = "Switch back to gnu toolchain"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command nightly-linux-gnu")
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
        powerShell {
            name = "Install toolchain"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command install-rustup-linux-gnu")
        }
        powerShell {
            name = "Set toolchain"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command nightly-linux-gnu")
        }
        powerShell {
            name = "Update toolchain"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command update")
        }
        powerShell {
            name = "Format check"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command format-check")
        }
        powerShell {
            name = "Clippy!"
            enabled = false
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command clippy")
        }
        powerShell {
            name = "Test"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command test")
        }
        powerShell {
            name = "Convert test output to junit"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command junit")
        }
        powerShell {
            name = "Convert junit to html"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command junit2html")
        }
        powerShell {
            name = "Grcov"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command grcov")
        }
        powerShell {
            name = "Generate coverage report"
            platform = PowerShellStep.Platform.x64
            edition = PowerShellStep.Edition.Core
            scriptMode = file {
                path = "tools/CI.ps1"
            }
            param("jetbrains_powershell_scriptArguments", "-Command coverage")
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
