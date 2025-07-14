<script lang="ts">
  // import { invoke } from "@tauri-apps/api/core";
  import type { PageProps } from "./$types";
  let today = new Date().toISOString().split("T")[0];
  let { data }: PageProps = $props();
</script>

<main class="container">
  <p>Loading...</p>
</main>

<style>

  :global(.app) {
    background-color: var(--background-color);
    width: 400px;
    height: 700px;
    margin: 0;
  }
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #232333;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
    --primary-color: #3498db;
    --btn-highlight-color: #65b7ee;
    --secondary-color: #141c28;
    --background-color: #232333;
    --text-color: #ddd;
    --border-color: #ddd;
    --button-padding: 10px 20px;
    --border-radius: 10px;

    --office-color: yellow;
    --home-color: yellowgreen;
    --nowork-color: rgb(56, 56, 56);
    --unknown-color: darkgray;

    --calendar-lines: rgb(56, 56, 56);
  }

  :global(.container) {
    margin: 0;
    padding-top: 5vh;
    display: flexbox;
    /* flex-direction: column; */
    justify-content: center;
    text-align: center;
    /* border-radius: 100px; */
    background-color: var(--background-color);
  }

  :global(.btn-flat) {
    display: inline-block;
    border: none;
    box-shadow: none;
    outline: none;
    background-color: var(--primary-color);
    color: white;
    border-radius: var(--border-radius);
    cursor: pointer;
    font-size: 14px;
    transition: background-color 0.2s ease;
    padding: 1em;
  }
  :global(.btn-flat:hover){
background-color: var(--btn-highlight-color);

  }
  :global(.row) {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    text-align: center;
    margin-bottom: 3vh;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: var(--background-color);
    }


  }
</style>
