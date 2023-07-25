<script lang="ts">
  const buildDate = __FBRAGE_BUILD_DATE__;
</script>

<div class="wrapper">
  <div class="menu">
    <div>
      <a role="button" href="/encrypt">Encrypt Message</a>
      <a role="button" href="/decrypt">Decrypt Message</a>
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
    <div>
      <span class="build-date">Build date: {buildDate}</span>
    </div>
  </div>
</div>

<style lang="scss">
  .build-date {
    font-size: 12px;
  }
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
      text-align: center;
    }
  }
</style>
