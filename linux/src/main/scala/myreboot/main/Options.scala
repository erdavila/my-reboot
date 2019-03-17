package myreboot.main

import myreboot.{OSPlatform, Platform}

object Options extends OptionsBase {
  override protected val platform: OSPlatform = Platform
}
