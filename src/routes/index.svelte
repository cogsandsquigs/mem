<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  //import dayjs from "dayjs";

  let reminders;
  onMount(async () => {
    reminders = await invoke("get_all_cards");
    console.log(reminders);
  });
</script>

<h1>mem</h1>
