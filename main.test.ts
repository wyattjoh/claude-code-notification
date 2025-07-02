import {
  assertEquals,
  assertRejects,
} from "https://deno.land/std@0.224.0/assert/mod.ts";
import { describe, it } from "https://deno.land/std@0.224.0/testing/bdd.ts";
import { main } from "./main.ts";

describe("main", () => {
  it("should parse valid JSON input and execute osascript command", async () => {
    const validInput = {
      session_id: "test-session-123",
      transcript_path: "/path/to/transcript.md",
      message: "Test notification message",
      title: "Test Title",
    };

    const originalCommand = Deno.Command;

    let commandExecuted = false;
    let commandArgs: string[] = [];

    // Mock Deno.Command
    // deno-lint-ignore no-explicit-any
    (Deno as any).Command = class MockCommand {
      constructor(cmd: string, options: { args: string[] }) {
        assertEquals(cmd, "osascript");
        commandArgs = options.args;
        commandExecuted = true;
      }

      output() {
        return Promise.resolve({
          success: true,
          code: 0,
          stdout: new Uint8Array(),
          stderr: new Uint8Array(),
        });
      }
    };

    // Create a ReadableStream with the test data
    const encoder = new TextEncoder();
    const inputData = encoder.encode(JSON.stringify(validInput));

    const stdin = new ReadableStream({
      start(controller) {
        controller.enqueue(inputData);
        controller.close();
      },
    });

    await main(stdin);

    // Verify command was executed
    assertEquals(commandExecuted, true);
    assertEquals(commandArgs.length, 2);
    assertEquals(commandArgs[0], "-e");
    assertEquals(
      commandArgs[1],
      `display notification "Test notification message" with title "Test Title" sound name "Glass"`,
    );

    // Restore original objects
    // deno-lint-ignore no-explicit-any
    (Deno as any).Command = originalCommand;
  });

  it("should handle special characters in message and title", async () => {
    const inputWithSpecialChars = {
      session_id: "test-session-456",
      transcript_path: "/path/to/transcript.md",
      message: 'Message with "quotes" and special chars',
      title: 'Title with "quotes"',
    };

    const originalCommand = Deno.Command;

    let commandArgs: string[] = [];

    // Mock Deno.Command
    // deno-lint-ignore no-explicit-any
    (Deno as any).Command = class MockCommand {
      constructor(_cmd: string, options: { args: string[] }) {
        commandArgs = options.args;
      }

      output() {
        return Promise.resolve({
          success: true,
          code: 0,
          stdout: new Uint8Array(),
          stderr: new Uint8Array(),
        });
      }
    };

    // Create a ReadableStream with the test data
    const encoder = new TextEncoder();
    const inputData = encoder.encode(JSON.stringify(inputWithSpecialChars));

    const stdin = new ReadableStream({
      start(controller) {
        controller.enqueue(inputData);
        controller.close();
      },
    });

    await main(stdin);

    // Verify the command handles special characters
    assertEquals(
      commandArgs[1],
      `display notification "Message with "quotes" and special chars" with title "Title with "quotes"" sound name "Glass"`,
    );

    // Restore original objects
    // deno-lint-ignore no-explicit-any
    (Deno as any).Command = originalCommand;
  });

  it("should throw error for invalid JSON input", async () => {
    // Create a ReadableStream with invalid JSON
    const encoder = new TextEncoder();
    const invalidJson = encoder.encode("{ invalid json }");

    const stdin = new ReadableStream({
      start(controller) {
        controller.enqueue(invalidJson);
        controller.close();
      },
    });

    await assertRejects(async () => await main(stdin), SyntaxError);
  });

  it("should handle empty stdin", async () => {
    // Create a ReadableStream with empty data
    const stdin = new ReadableStream({
      start(controller) {
        controller.close();
      },
    });

    await assertRejects(async () => await main(stdin), SyntaxError);
  });

  it("should handle missing required fields in input", async () => {
    const incompleteInput = {
      session_id: "test-session-789",
      // Missing other required fields
    };

    const originalCommand = Deno.Command;

    let commandExecuted = false;

    // Mock Deno.Command
    // deno-lint-ignore no-explicit-any
    (Deno as any).Command = class MockCommand {
      constructor() {
        commandExecuted = true;
      }

      output() {
        return Promise.resolve({
          success: true,
          code: 0,
          stdout: new Uint8Array(),
          stderr: new Uint8Array(),
        });
      }
    };

    // Create a ReadableStream with the test data
    const encoder = new TextEncoder();
    const inputData = encoder.encode(JSON.stringify(incompleteInput));

    const stdin = new ReadableStream({
      start(controller) {
        controller.enqueue(inputData);
        controller.close();
      },
    });

    await main(stdin);

    // Command should still execute even with undefined fields
    assertEquals(commandExecuted, true);

    // Restore original objects
    // deno-lint-ignore no-explicit-any
    (Deno as any).Command = originalCommand;
  });

  it("should use custom sound when provided", async () => {
    const validInput = {
      session_id: "test-session-789",
      transcript_path: "/path/to/transcript.md",
      message: "Custom sound notification",
      title: "Test Title",
    };

    const originalCommand = Deno.Command;

    let commandArgs: string[] = [];

    // Mock Deno.Command
    // deno-lint-ignore no-explicit-any
    (Deno as any).Command = class MockCommand {
      constructor(cmd: string, options: { args: string[] }) {
        assertEquals(cmd, "osascript");
        commandArgs = options.args;
      }

      output() {
        return Promise.resolve({
          success: true,
          code: 0,
          stdout: new Uint8Array(),
          stderr: new Uint8Array(),
        });
      }
    };

    // Create a ReadableStream with the test data
    const encoder = new TextEncoder();
    const inputData = encoder.encode(JSON.stringify(validInput));

    const stdin = new ReadableStream({
      start(controller) {
        controller.enqueue(inputData);
        controller.close();
      },
    });

    // Test with custom sound
    await main(stdin, "Submarine");

    // Verify the command uses custom sound
    assertEquals(
      commandArgs[1],
      `display notification "Custom sound notification" with title "Test Title" sound name "Submarine"`
    );

    // Restore original objects
    // deno-lint-ignore no-explicit-any
    (Deno as any).Command = originalCommand;
  });

  it("should use default Glass sound when not specified", async () => {
    const validInput = {
      session_id: "test-session-999",
      transcript_path: "/path/to/transcript.md",
      message: "Default sound notification",
      title: "Test Title",
    };

    const originalCommand = Deno.Command;

    let commandArgs: string[] = [];

    // Mock Deno.Command
    // deno-lint-ignore no-explicit-any
    (Deno as any).Command = class MockCommand {
      constructor(cmd: string, options: { args: string[] }) {
        assertEquals(cmd, "osascript");
        commandArgs = options.args;
      }

      output() {
        return Promise.resolve({
          success: true,
          code: 0,
          stdout: new Uint8Array(),
          stderr: new Uint8Array(),
        });
      }
    };

    // Create a ReadableStream with the test data
    const encoder = new TextEncoder();
    const inputData = encoder.encode(JSON.stringify(validInput));

    const stdin = new ReadableStream({
      start(controller) {
        controller.enqueue(inputData);
        controller.close();
      },
    });

    // Test without specifying sound (should default to Glass)
    await main(stdin);

    // Verify the command uses default Glass sound
    assertEquals(
      commandArgs[1],
      `display notification "Default sound notification" with title "Test Title" sound name "Glass"`
    );

    // Restore original objects
    // deno-lint-ignore no-explicit-any
    (Deno as any).Command = originalCommand;
  });
});
