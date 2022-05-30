<script setup lang="ts">
import { ref } from "vue"
</script>

<template>
  <div class="hi-about">
    <div class="container">
      <h1>About the project</h1>
      <p>
        This project was created in collaboration of friends from the hivecom community, which also formed its own
        development team called <a target="_blank" href="https://github.com/mavulp">Mavulp</a>. The goal for us was to
        create a centralized hub to share albums or iamges mostly for real life events or meetings. <br />
        <br />

        Given the fact we've all grown in different parts of the world and some of us have knonw each other for over 10
        years, these albums can hold a lot of sentimntal value.

        <br />
        <br />
        <br />

        We also do a lot of gaming together, want to join? Check out our site
      </p>

      <a target="_blank" href="https://hivecom.net/" class="link-out">
        <div>
          <strong>Hivecom Community</strong>
          <p>https://hivecom.net/</p>
        </div>

        <img src="https://hivecom.net/public/metaicon2.jpg" alt="" />
      </a>

      <hr />

      <h3>Suggestions</h3>
      <p>
        Did you find a bug or thought of a cool feature which would enhance our site? You can create a
        <a target="_blank" href="https://github.com/Mavulp/hivefriends/issues">Github issue</a> or send us a message on
        <a target="_blank" href="https://discord.com/invite/Uh4WJ6vV">Discord</a> in the
        <strong>#lounge</strong> channel.
      </p>

      <p class="copyright">
        <span class="material-icons"> &#xe86f; </span>
        Made by <a target="_blank" href="https://github.com/mavulp">Mavulp</a> in {{ new Date().getFullYear() }}
      </p>
    </div>
  </div>
</template>
