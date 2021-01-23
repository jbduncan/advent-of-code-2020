package org.jbduncan.adventofcode2020.day7.util

import com.google.common.base.StandardSystemProperty
import java.nio.file.Path
import org.jbduncan.adventofcode2020.day7.AppVersion

object AppRunner {
  private val VERSION = AppVersion.get()

  fun runApp(vararg args: Any): Result {
    val process =
        ProcessBuilder(
                listOf(
                    Path.of(StandardSystemProperty.JAVA_HOME.value())
                        .resolve("bin/java")
                        .toString(),
                    "-jar",
                    "build/libs/day-7-jvm-${VERSION}-all.jar",
                ) + args.map(Any::toString))
            .redirectError(ProcessBuilder.Redirect.PIPE)
            .redirectOutput(ProcessBuilder.Redirect.PIPE)
            .start()

    val exitCode = process.waitFor()
    val stdOut = process.inputStream.bufferedReader().use { w -> w.readText() }
    val stdErr = process.errorStream.bufferedReader().use { w -> w.readText() }

    return Result(exitCode, stdOut, stdErr)
  }

  data class Result(val exitCode: Int, val stdOut: String, val stdErr: String)
}
