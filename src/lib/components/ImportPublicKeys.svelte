<script lang="ts">
  import ErrorMessage from "./ErrorMessage.svelte";
  import { publicKeyStore, type PublicKeyStore } from "$lib/public_key_store";
  import { get } from "svelte/store";

  let error: string | undefined = $state();

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
    error = undefined;
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
              keys: window
                .atob(fileDetails.content)
                .split("\n")
                .filter(Boolean)
                .filter((s) => !s.trim().startsWith("#")),
            };
          }
          return null;
        }),
    );

    const importedPublicKeys: PublicKeyStore = publicKeys
      .filter((i): i is UserKeys => i != null)
      .reduce((acc, val) => {
        return {
          ...acc,
          [val.user]: [...(acc[val.user] || []), ...val.keys],
        };
      }, {} as PublicKeyStore);

    const currentPublicKeys = get(publicKeyStore);

    publicKeyStore.set({
      ...currentPublicKeys,
      ...importedPublicKeys,
    });
  }
</script>

<article>
  <header>Import symbiolab public keys</header>
  <p>Import will overwrite existing symbiolab recipients. Recipients that are not part of symbiolab are left intact.</p>
  <button onclick={loadPublicKeys}>Import</button>
  {#if error != null}
    <ErrorMessage {error} />
  {/if}
</article>
