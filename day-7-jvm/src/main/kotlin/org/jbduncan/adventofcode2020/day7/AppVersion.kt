package org.jbduncan.adventofcode2020.day7

import com.google.common.io.Resources.getResource
import com.google.common.io.Resources.toString
import kotlin.text.Charsets.UTF_8

object AppVersion {
  private val version: String by lazy { toString(getResource("version.txt"), UTF_8).trim() }

  fun get(): String {
    return version
  }
}
