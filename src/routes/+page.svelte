<div class="wrapper">
  <div class="menu">
    <div>
      <a role="button" href="/decrypt">Decrypt Message</a>
      <a role="button" href="/encrypt">Encrypt Message</a>
    </div>
    <div>
      <a role="button" href="/encrypt-file">Encrypt File</a>
      <a role="button" href="/decrypt-file">Decrypt File</a>
    </div>
    <div>
      <a role="button" href="/private-key">Private key management</a>
      <a role="button" href="/public-keys">Public key management</a>
      <a role="button" href="/gen-key">Generate Key</a>
    </div>
  </div>
</div>

<style lang="scss">
  .wrapper {
    display: flex;
    justify-content: center;
  }
  .menu {
    display: grid;
    gap: 30px;
    max-width: 600px;

    > div {
      display: flex;
      gap: 10px;
      flex-direction: column;
    }
  }
</style>
