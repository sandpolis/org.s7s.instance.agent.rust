//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

rootProject.name = "com.sandpolis.agent.micro"

sourceControl {
	gitRepository("https://github.com/sandpolis/com.sandpolis.core.foundation.git") {
		producesModule("com.sandpolis:core.foundation")
	}
}
sourceControl {
	gitRepository("https://github.com/sandpolis/com.sandpolis.core.instance.git") {
		producesModule("com.sandpolis:core.instance")
	}
}
sourceControl {
	gitRepository("https://github.com/sandpolis/com.sandpolis.core.net.git") {
		producesModule("com.sandpolis:core.net")
	}
}
sourceControl {
	gitRepository("https://github.com/sandpolis/com.sandpolis.plugin.snapshot.git") {
		producesModule("com.sandpolis:plugin.snapshot")
	}
}

include(":module:com.sandpolis.core.foundation")
