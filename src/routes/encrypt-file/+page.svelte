<script lang="ts">
  import PublicKeyPicker from "$lib/components/PublicKeyPicker.svelte";
  import { encrypt_file } from "rage-webassembly";
  import { publicKeyStore } from "$lib/public_key_store";
  import { get } from "svelte/store";
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
  import { onDestroy } from "svelte";

  let file: File | null;
  let error: string | null = null;
  let selection: string[] = [];
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
        const publicKeys = selection.flatMap((s) => get(publicKeyStore)[s]);
        if (event.target && event.target.result instanceof ArrayBuffer) {
          const contents = new Uint8Array(event.target.result);
          try {
            const result = encrypt_file(contents, publicKeys);
            const blob = new Blob([result], { type: "application/octet-stream" });
            objectURL = URL.createObjectURL(blob);
            const link = document.createElement("a");
            link.href = objectURL;
            link.download = `${file?.name}.age`;
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

<form on:submit|preventDefault={handleSubmit}>
  <PublicKeyPicker bind:selection />
  <input required type="file" on:change={handleChange} />
  <button>Encrypt</button>
</form>

{#if error != null}
  <ErrorMessage {error} />
{/if}
