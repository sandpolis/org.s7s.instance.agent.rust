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
	id("sandpolis-instance")
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
			proto("com.sandpolis:${it.first()}.${it.last()}:+:rust@zip")
		}
	}

	val assembleProto by tasks.creating(Copy::class) {
		into("src/gen/rust")

		proto.files.forEach { dep ->
			from(zipTree(dep))
		}
	}

} else {

	val assembleProto by tasks.creating(Copy::class) {
		into("src/gen/rust")

		for (dep in protoDependencies) {
			from(project(dep).file("src/gen/rust"))
		}
	}
}

val buildLinuxAmd64 by tasks.creating(Exec::class) {
	dependsOn(tasks.findByName("assembleProto"))
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--bin", "agent", "--bin", "bootagent", "--target=x86_64-unknown-linux-gnu"))
	outputs.file("target/x86_64-unknown-linux-gnu/release/bootagent", "target/x86_64-unknown-linux-gnu/release/agent")
}

val buildLinuxAarch64 by tasks.creating(Exec::class) {
	dependsOn(tasks.findByName("assembleProto"))
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--bin", "agent", "--bin", "bootagent", "--target=aarch64-unknown-linux-gnu"))
	outputs.file("target/aarch64-unknown-linux-gnu/release/bootagent", "target/aarch64-unknown-linux-gnu/release/agent")
}

val buildLinuxArmv7 by tasks.creating(Exec::class) {
	dependsOn(tasks.findByName("assembleProto"))
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--bin", "agent", "--bin", "bootagent", "--target=armv7-unknown-linux-musleabihf"))
	outputs.file("target/armv7-unknown-linux-musleabihf/release/bootagent", "target/armv7-unknown-linux-musleabihf/release/agent")
}

val buildWindowsAmd64 by tasks.creating(Exec::class) {
	dependsOn(tasks.findByName("assembleProto"))
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--bin", "agent", "--target=x86_64-pc-windows-gnu"))
	outputs.file("target/x86_64-pc-windows-gnu/release/agent")
}

tasks.findByName("build")?.dependsOn(buildLinuxAmd64, buildLinuxAarch64, buildLinuxArmv7, buildWindowsAmd64)

publishing {
	publications {
		create<MavenPublication>("agent") {
			groupId = "com.sandpolis"
			artifactId = "agent.micro"
			version = project.version.toString()

			artifact(buildLinuxAmd64.outputs.files.filter { it.name == "agent" }.getSingleFile()) {
				classifier = "linux-amd64"
			}

			artifact(buildLinuxAarch64.outputs.files.filter { it.name == "agent" }.getSingleFile()) {
				classifier = "linux-aarch64"
			}

			artifact(buildLinuxArmv7.outputs.files.filter { it.name == "agent" }.getSingleFile()) {
				classifier = "linux-armv7"
			}

			artifact(buildWindowsAmd64.outputs.files.getSingleFile()) {
				classifier = "windows-amd64"
			}
		}
		tasks.findByName("publishAgentPublicationToGitHubPackagesRepository")?.dependsOn("build")

		create<MavenPublication>("bootagent") {
			groupId = "com.sandpolis"
			artifactId = "agent.boot"
			version = project.version.toString()

			artifact(buildLinuxAmd64.outputs.files.filter { it.name == "bootagent" }.getSingleFile()) {
				classifier = "linux-amd64"
			}

			artifact(buildLinuxAarch64.outputs.files.filter { it.name == "bootagent" }.getSingleFile()) {
				classifier = "linux-aarch64"
			}

			artifact(buildLinuxArmv7.outputs.files.filter { it.name == "bootagent" }.getSingleFile()) {
				classifier = "linux-armv7"
			}
		}
		tasks.findByName("publishBootagentPublicationToGitHubPackagesRepository")?.dependsOn("build")
	}
}
