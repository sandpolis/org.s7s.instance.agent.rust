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
}

if (project.getParent() == null) {
	task("generateProto") {
		// TODO
	}
} else {
	task("generateProto") {
		dependsOn(project(":module:com.sandpolis.core.foundation").tasks.findByName("generateProto"))
		dependsOn(project(":module:com.sandpolis.core.instance").tasks.findByName("generateProto"))
		dependsOn(project(":module:com.sandpolis.core.net").tasks.findByName("generateProto"))
	}
}
