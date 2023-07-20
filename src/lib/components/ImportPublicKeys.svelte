<script lang="ts">
  import ErrorMessage from "./ErrorMessage.svelte";
  import { publicKeyStore, type PublicKeyStore } from "$lib/public_key_store";

  let error: string | null;

  interface RepositoryTree {
    tree: GithubFile[];
  }

  interface GithubFile {
    url: string;
    path: string;
  }

  interface GithubFileDetails {
    content: string;
  }

  interface UserKeys {
    user: string;
    keys: string[];
  }

  async function loadPublicKeys() {
    error = null;
    const response = await fetch("https://api.github.com/repos/symbiolab/age-public-keys/git/trees/main?recursive=0");

    if (!response.ok) {
      error = response.statusText;
      return;
    }

    const data: RepositoryTree = await response.json();

    const publicKeys: (UserKeys | null)[] = await Promise.all(
      data.tree
        .filter((i) => i.path.endsWith("age.pub"))
        .map(async (file) => {
          const response = await fetch(file.url);
          if (response.ok) {
            const fileDetails: GithubFileDetails = await response.json();
            return {
              user: file.path.replace(".age.pub", ""),
              keys: window.atob(fileDetails.content).split("\n").filter(Boolean),
            };
          }
          return null;
        }),
    );

    const newPublicKeystore: PublicKeyStore = publicKeys
      .filter((i): i is UserKeys => i != null)
      .reduce((acc, val) => {
        return {
          ...acc,
          [val.user]: [...(acc[val.user] || []), ...val.keys],
        };
      }, {} as PublicKeyStore);

    publicKeyStore.set(newPublicKeystore);
  }
</script>

<button on:click={loadPublicKeys}>Import Public Keys</button>
{#if error != null}
  <ErrorMessage {error} />
{/if}
