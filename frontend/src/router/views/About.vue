<script setup lang="ts">
import { onBeforeMount, ref } from "vue"
import { useBread } from "../../store/bread"

const bread = useBread()

onBeforeMount(() => {
  bread.set("About the project")
})
</script>

<template>
  <div class="hi-about">
    <div class="container">
      <h1>The project</h1>
      <p>
        This project was created as a collaboration of friends from the Hivecom community and the dedicated development
        team called <a target="_blank" href="https://github.com/mavulp">Mavulp</a>. The goal of Hi!Friends is to create
        a centralized hub to share images of mostly real life gatherings, events and adventures we embark on together.
        Given the fact we've all grown in different parts of the world and some of us have knonw each other for over 10
        years, these albums can hold a lot of sentimntal value.

        <br />
        <br />

        Given the fact that we - the community behind Hivecom and it's further groups of friends - all grew up in
        different parts of the world and in the course of over 10 years found together through our shared interests,
        goals and quite simply bizarre circumstances; these albums can hold a lot of sentimental value that can only be
        described as a family bond.

        <br />
        <br />
        We'd like to share this bond between all of us, and all of you who might one day be a part of that too. In the
        meantime: we also do a lot of gaming together, want to join? Check out our site!
      </p>

      <a target="_blank" href="https://hivecom.net/" class="link-out">
        <div>
          <strong>Hivecom Community</strong>
          <p>https://hivecom.net/</p>
        </div>

        <img src="https://hivecom.net/public/metaicon2.jpg" alt="" />
      </a>

      <!-- <hr /> -->

      <h3>Suggestions</h3>
      <p>
        Did you find a bug or thought of a cool feature which would enhance our site? You can create a
        <a target="_blank" href="https://github.com/Mavulp/hivefriends/issues">Github issue</a> or send us a message on
        <a target="_blank" href="https://discord.com/invite/Uh4WJ6vV">Discord</a> in the <strong>#chat</strong> channel.
      </p>
      <br />
      <br />
      <br />
      <br />
      <br />

      <h4>Support us</h4>

      <p>
        Running all the servers and storage adds to cost each month. We do all the financing out of our own pockets. If
        you want to support us, feel free to donate a small amount <br />
        to our <a href="https://ko-fi.com/hivecom">ko-fi</a> or join our patreon.
      </p>

      <a target="_blank" href="https://www.patreon.com/hivecom" class="link-out">
        <div>
          <strong>Hivecom on Patreon</strong>
          <p>https://www.patreon.com/hivecom</p>
        </div>

        <img
          src="https://i0.wp.com/jamesgarside.net/wp-content/uploads/2019/03/e04cb391687ead757f671bc977b9de83.jpg"
          alt=""
        />
      </a>

      <p class="copyright">
        <span class="material-icons"> &#xe86f; </span>
        Made by <a target="_blank" href="https://github.com/mavulp">Mavulp</a> in {{ new Date().getFullYear() }}
      </p>
    </div>
  </div>
</template>
