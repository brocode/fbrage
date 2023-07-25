export async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text);
    console.log("Text copied to clipboard");
  } catch (err) {
    console.log("Failed to copy text: ", err);
  }
}
