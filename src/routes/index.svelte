<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { emit, listen } from "@tauri-apps/api/event";

  //import dayjs from "dayjs";

  // where all the logic goes
  onMount(async () => {
    // listen to the `click` event and get a function to remove the event listener
    // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
    const unlisten = await listen("add-card", (event) => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
    });

    // emits the `click` event with the object payload
    emit("add-card", {
      id: 0,
      topic: "", // the cards' topic
      front: "", // the cards' front face, as a markdown string
      back: "", // the cards' back face, as a markdown string
      bucket: 0, // the bucket the card is in - used for the leitner system
      date_created: "",
    });
  });
</script>

<h1>mem</h1>
