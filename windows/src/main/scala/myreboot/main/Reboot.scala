package myreboot.main

import myreboot.{OSPlatform, Platform}

object Reboot extends RebootBase {
  override protected val platform: OSPlatform = Platform
}
