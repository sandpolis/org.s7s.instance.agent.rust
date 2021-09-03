//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

plugins {
	id("sandpolis-module")
	id("sandpolis-soi")
	id("sandpolis-publish")
}

val protoDependencies = listOf(
	":module:com.sandpolis.core.foundation",
	":module:com.sandpolis.core.instance",
	":module:com.sandpolis.core.net",
	":plugin:com.sandpolis.plugin.snapshot"
)

if (project.getParent() == null) {
	repositories {
		protoDependencies.map { it.split(".").takeLast(2) }.forEach {
			maven("https://maven.pkg.github.com/sandpolis/com.sandpolis.${it.first()}.${it.last()}") {
				credentials {
					username = System.getenv("GITHUB_ACTOR")
					password = System.getenv("GITHUB_TOKEN")
				}
			}
		}
	}

	val proto by configurations.creating

	dependencies {
		protoDependencies.map { it.split(".").takeLast(2) }.forEach {
			proto("com.sandpolis:${it.first()}.${it.last()}-rust:+@zip")
		}
	}

	val assembleProto by tasks.creating(Copy::class) {

		into("src/main/rust/gen")

		proto.files.forEach { dep ->
			var path = dep.absolutePath.split("-").takeLast(3)
			path = path.first().split("\\.|/".toRegex()).takeLast(2)

			into("com.sandpolis.${path[0]}.${path[1]}") {
				with(copySpec {
					from(zipTree(dep))
				})
			}
		}
	}

} else {

	val assembleProto by tasks.creating(Copy::class) {

		into("src/main/rust/gen")

		for (dep in protoDependencies) {

			into(dep.substring(dep.lastIndexOf(":") + 1)) {
				with (copySpec {
					from(project(dep).file("gen/main/rust"))
				})
			}
		}
	}
}

val buildLinuxAmd64 by tasks.creating(Exec::class) {
	dependsOn(tasks.findByName("assembleProto"))
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--bin", "agent", "--bin", "bootagent", "--target=x86_64-unknown-linux-gnu"))
}
tasks.findByName("build")?.dependsOn(buildLinuxAmd64)

val buildLinuxAarch64 by tasks.creating(Exec::class) {
	dependsOn(tasks.findByName("assembleProto"))
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--bin", "agent", "--bin", "bootagent", "--target=aarch64-unknown-linux-gnu"))
}
tasks.findByName("build")?.dependsOn(buildLinuxAarch64)

val buildLinuxArmv7 by tasks.creating(Exec::class) {
	dependsOn(tasks.findByName("assembleProto"))
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--bin", "agent", "--bin", "bootagent", "--target=armv7-unknown-linux-musleabihf"))
}
tasks.findByName("build")?.dependsOn(buildLinuxArmv7)

val buildWindowsAmd64 by tasks.creating(Exec::class) {
	dependsOn(tasks.findByName("assembleProto"))
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--bin", "agent", "--target=x86_64-pc-windows-gnu"))
}
tasks.findByName("build")?.dependsOn(buildWindowsAmd64)

publishing {
	publications {
		create<MavenPublication>("mavenAgentLinuxAmd64") {
			groupId = "com.sandpolis"
			artifactId = "agent-linux-amd64"
			version = project.version.toString()

			artifact(project.file("target/x86_64-unknown-linux-gnu/release/agent"))
		}
		tasks.findByName("mavenAgentLinuxAmd64")?.dependsOn(buildLinuxAmd64)

		create<MavenPublication>("mavenBootagentLinuxAmd64") {
			groupId = "com.sandpolis"
			artifactId = "bootagent-linux-amd64"
			version = project.version.toString()

			artifact(project.file("target/x86_64-unknown-linux-gnu/release/bootagent"))
		}
		tasks.findByName("mavenBootagentLinuxAmd64")?.dependsOn(buildLinuxAmd64)

		create<MavenPublication>("mavenAgentLinuxAarch64") {
			groupId = "com.sandpolis"
			artifactId = "agent-linux-aarch64"
			version = project.version.toString()

			artifact(project.file("target/aarch64-unknown-linux-gnu/release/agent"))
		}
		tasks.findByName("mavenAgentLinuxAarch64")?.dependsOn(buildLinuxAmd64)

		create<MavenPublication>("mavenBootagentLinuxAarch64") {
			groupId = "com.sandpolis"
			artifactId = "bootagent-linux-aarch64"
			version = project.version.toString()

			artifact(project.file("target/aarch64-unknown-linux-gnu/release/bootagent"))
		}
		tasks.findByName("mavenBootagentLinuxAarch64")?.dependsOn(buildLinuxAmd64)
	}
}
