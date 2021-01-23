package org.jbduncan.adventofcode2020.day7

import java.io.IOException
import java.nio.file.Files.readString
import java.nio.file.Path
import java.util.concurrent.Callable
import kotlin.system.exitProcess
import picocli.CommandLine
import picocli.CommandLine.Command
import picocli.CommandLine.Model.CommandSpec
import picocli.CommandLine.Option
import picocli.CommandLine.Parameters
import picocli.CommandLine.Spec

fun main(args: Array<String>) {
  exitProcess(
      CommandLine(App())
          .also { cmd -> cmd.executionExceptionHandler = PrintExceptionMessageHandler(cmd) }
          .execute(*args))
}

@Command(
    name = "day-7-jvm",
    mixinStandardHelpOptions = true,
    versionProvider = PicocliVersionProvider::class)
class App : Callable<Int> {

  @Spec private lateinit var spec: CommandSpec

  @Parameters(index = "0", description = ["The input file to use."]) private lateinit var file: Path

  @Option(
      names = ["-p", "--part-2"],
      description =
          ["Should we return the result for part 2? If not, we return the result for part 1."])
  private var part2: Boolean = false

  override fun call(): Int {
    val rules =
        try {
          readString(file)
        } catch (ex: IOException) {
          throw UserFacingException("Failed to read file ${file}.")
        }
    if (!part2) {
      printLine(BagRules.parse(rules).uniqueBagsContaining("shiny gold bag"))
    } else {
      printLine(BagRules.parse(rules).bagsContainedBy("shiny gold bag"))
    }
    return 0
  }

  private fun printLine(value: Any?) {
    spec.commandLine().out.println(value)
  }
}

class PrintExceptionMessageHandler(private val commandLine: CommandLine) :
    CommandLine.IExecutionExceptionHandler {

  override fun handleExecutionException(
      ex: Exception,
      cmd: CommandLine,
      parseResult: CommandLine.ParseResult
  ): Int {
    if (ex is UserFacingException) {
      ex.message?.let(this::errPrintLine)

      return commandLine.let {
        it.exitCodeExceptionMapper?.getExitCode(ex) ?: it.commandSpec.exitCodeOnExecutionException()
      }
    }

    throw ex
  }

  private fun errPrintLine(value: Any?) {
    commandLine.err.println(commandLine.colorScheme.errorText(value.toString()))
  }
}

class PicocliVersionProvider : CommandLine.IVersionProvider {
  override fun getVersion(): Array<String> {
    return arrayOf(AppVersion.get())
  }
}
