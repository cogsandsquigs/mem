<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { emit, listen } from "@tauri-apps/api/event";

  //import dayjs from "dayjs";

  // where all the logic goes
  onMount(async () => {
    let reminders = await invoke("get_all_cards");
    console.log(reminders);

    // listen to the `click` event and get a function to remove the event listener
    // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
    const unlisten = await listen("add-card", (event) => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
    });

    // emits the `click` event with the object payload
    emit("add-card", {
      theMessage: "Tauri is awesome!",
    });
  });
</script>

<h1>mem</h1>
