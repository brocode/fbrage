<script lang="ts">
  import { decrypt_file } from "rage-webassembly";
  import { get } from "svelte/store";
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
  import { onDestroy } from "svelte";
  import { privateKeyStore } from "$lib/private_key_store";

  let file: File | null;
  let error: string | null = null;
  let objectURL: string | null = null;

  onDestroy(() => {
    objectURL && URL.revokeObjectURL(objectURL);
  });

  function handleChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const files = target?.files;
    file = files ? files[0] : null;
  }

  function handleSubmit() {
    error = null;
    if (objectURL) {
      URL.revokeObjectURL(objectURL);
      objectURL = null;
    }
    if (file) {
      const reader = new FileReader();

      reader.onload = function (event) {
        if (event.target && event.target.result instanceof ArrayBuffer) {
          const contents = new Uint8Array(event.target.result);
          try {
            const result = decrypt_file(contents, Object.values(get(privateKeyStore)));
            const blob = new Blob([result], { type: "application/octet-stream" });
            objectURL = URL.createObjectURL(blob);
            const link = document.createElement("a");
            link.href = objectURL;
            const fileName = file?.name ?? "download";
            link.download = fileName.replace(/\.age$/, "");
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
          } catch (e) {
            error = String(e);
          }
        }
      };
      reader.readAsArrayBuffer(file);
    }
  }
</script>

<p>Available private keys: {Object.keys($privateKeyStore).length}</p>

<form on:submit|preventDefault={handleSubmit}>
  <input required type="file" on:change={handleChange} />
  <button>Decrypt</button>
</form>

{#if error != null}
  <ErrorMessage {error} />
{/if}
