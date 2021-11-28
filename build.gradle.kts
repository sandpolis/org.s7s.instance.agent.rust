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
	id("com.sandpolis.build.module")
	id("com.sandpolis.build.instance")
	id("com.sandpolis.build.publish")
}

dependencies {
	proto("com.sandpolis:core.foundation:+:rust@zip")
	proto("com.sandpolis:core.instance:+:rust@zip")
	proto("com.sandpolis:core.net:+:rust@zip")
	proto("com.sandpolis:plugin.snapshot:+:rust@zip")
}

val buildLinuxAmd64 by tasks.creating(Exec::class) {
	dependsOn("assembleProto")
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--target=x86_64-unknown-linux-gnu"))
	outputs.files("target/x86_64-unknown-linux-gnu/release/agent")
}

val buildLinuxAarch64 by tasks.creating(Exec::class) {
	dependsOn("assembleProto")
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--target=aarch64-unknown-linux-gnu"))
	outputs.files("target/aarch64-unknown-linux-gnu/release/agent")
}

val buildLinuxArmv7 by tasks.creating(Exec::class) {
	dependsOn("assembleProto")
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--target=armv7-unknown-linux-musleabihf"))
	outputs.files("target/armv7-unknown-linux-musleabihf/release/agent")
}

val buildWindowsAmd64 by tasks.creating(Exec::class) {
	dependsOn("assembleProto")
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--target=x86_64-pc-windows-gnu"))
	outputs.files("target/x86_64-pc-windows-gnu/release/agent")
}

tasks.findByName("build")?.dependsOn(buildLinuxAmd64, buildLinuxAarch64, buildLinuxArmv7, buildWindowsAmd64)

publishing {
	publications {
		create<MavenPublication>("agent") {
			groupId = "com.sandpolis"
			artifactId = "agent.micro"
			version = project.version.toString()

			artifact("target/x86_64-unknown-linux-gnu/release/agent") {
				classifier = "linux-amd64"
			}

			artifact("target/aarch64-unknown-linux-gnu/release/agent") {
				classifier = "linux-aarch64"
			}

			artifact("target/armv7-unknown-linux-musleabihf/release/agent") {
				classifier = "linux-armv7"
			}

			artifact("target/x86_64-pc-windows-gnu/release/agent") {
				classifier = "windows-amd64"
			}
		}
		tasks.findByName("publishAgentPublicationToCentralStagingRepository")?.dependsOn("build")
	}
}
