package org.jbduncan.adventofcode2020.day7

import com.google.common.io.MoreFiles.asCharSink
import com.google.common.io.Resources.asCharSource
import com.google.common.io.Resources.getResource
import com.google.common.truth.Truth.assertThat
import java.nio.file.Files.writeString
import java.nio.file.Path
import kotlin.text.Charsets.UTF_8
import org.jbduncan.adventofcode2020.day7.util.AppRunner
import org.jbduncan.adventofcode2020.day7.util.AppRunner.runApp
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Nested
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertAll
import org.junit.jupiter.api.io.TempDir

class AppTests {

  lateinit var inputFile: Path

  @BeforeEach
  fun setup(@TempDir rulesFileDir: Path) {
    this.inputFile = rulesFileDir.resolve("input.txt")
  }

  @Nested
  inner class GivenTheFirstExampleBagRules {

    @BeforeEach
    fun setup() {
      writeString(
          inputFile,
          """
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
          """.trimIndent())
    }

    @Nested
    inner class WhenFindingTheNumberOfBagsThatCanEventuallyContainAShinyGoldBag {

      @Test
      fun `then it outputs 4 to stdout`() {
        val result = runApp(inputFile)

        assertThat(result).isEqualTo(AppRunner.Result(0, "4\n", ""))
      }
    }

    @Nested
    inner class WhenFindingTheNumberOfBagsThatShinyGoldBAgsMustContain {

      @Test
      fun `then it outputs 32 to stdout`() {
        val result = runApp(inputFile, "--part-2")

        assertThat(result).isEqualTo(AppRunner.Result(0, "32\n", ""))
      }
    }
  }

  @Nested
  inner class GivenTheSecondExampleBagRules {

    @BeforeEach
    fun setup() {
      writeString(
          inputFile,
          """
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.
          """.trimIndent())
    }

    @Nested
    inner class WhenFindingTheNumberOfBagsThatShinyGoldBagsMustContain {

      @Test
      fun `then it outputs 126`() {
        val result = runApp(inputFile, "--part-2")

        assertThat(result).isEqualTo(AppRunner.Result(0, "126\n", ""))
      }
    }
  }

  @Nested
  inner class GivenTheActualPuzzleInput {

    @BeforeEach
    fun setup() {
      asCharSource(getResource("puzzle-input.txt"), UTF_8).copyTo(asCharSink(inputFile, UTF_8))
    }

    @Nested
    inner class WhenFindingTheNumberOfBagsThatShinyGoldBagsMustContain {

      @Test
      fun `then it outputs 2976`() {
        val result = runApp(inputFile, "--part-2")

        assertThat(result).isEqualTo(AppRunner.Result(0, "2976\n", ""))
      }
    }
  }

  @Nested
  inner class GivenSomeBadPuzzleInputWithAnInvalidRule {

    @BeforeEach
    fun setup() {
      writeString(inputFile, "crazy purple bags play bagpipes.")
    }

    @Nested
    inner class WhenFindingTheNumberOfBagsThatCanEventuallyContainAShinyGoldBag {

      @Test
      fun `then it outputs an error message`() {
        val result = runApp(inputFile)

        assertAll(
            { assertThat(result.exitCode).isEqualTo(1) },
            { assertThat(result.stdOut).isEmpty() },
            { assertThat(result.stdErr).contains("crazy purple bags play bagpipes.") },
            {
              assertThat(result.stdErr)
                  .contains(
                      "<adjective> <colour> bags contain <number> <adjective> <colour> (bag|bags).")
            })
      }
    }
  }
}
