type NotificationInput = {
  /**
   * The session ID of the Claude session.
   */
  session_id: string;

  /**
   * The path to the transcript of the Claude session.
   */
  transcript_path: string;

  /**
   * The message of the notification.
   */
  message: string;
  /**
   * The title of the notification.
   */
  title: string;
};

/**
 * Reads the notification payload input from Claude and displays a notification
 * to the user using AppleScript.
 */
export async function main(
  stdin: ReadableStream<Uint8Array>,
  sound: string = "Glass",
): Promise<void> {
  // Parse the input from stdin.
  const decoder = new TextDecoder();
  let chunks: string = "";
  for await (const chunk of stdin) {
    chunks += decoder.decode(chunk);
  }

  // Parse the input from stdin.
  const input = JSON.parse(chunks) as NotificationInput;

  // Send a notification to the user that the code has finished running.
  const command = new Deno.Command("osascript", {
    args: [
      "-e",
      `display notification "${input.message}" with title "${input.title}" sound name "${sound}"`,
    ],
  });

  // Run the command.
  await command.output();
}

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  // Parse command line arguments
  let sound = "Glass";
  const args = Deno.args;
  
  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--sound" && i + 1 < args.length) {
      sound = args[i + 1];
      break;
    }
  }

  main(Deno.stdin.readable, sound);
}
