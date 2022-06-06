<script lang="ts">
  import dayjs from "dayjs";
  import relativeTime from "dayjs/plugin/relativeTime";
  dayjs.extend(relativeTime);
  import { onMount, onDestroy } from "svelte";
  import { reminders, add, remove } from "$lib/reminder";

  // not needed (yet)
  // import utc from "dayjs/plugin/utc";
  // import timezone from "dayjs/plugin/timezone"; // dependent on utc plugin
  // dayjs.extend(utc);
  // dayjs.extend(timezone);

  $: red = dayjs(time).unix() - dayjs().unix() < 1 ? "color: red;" : "";

  // reminder exports
  export let id: number = 0; // the reminder's id
  export let name: string = "..."; // the name of the reminder
  export let time: number | dayjs.Dayjs | Date = dayjs()
    .hour(dayjs().hour() + 1)
    .minute(Math.round(dayjs().minute() / 5) * 5); // the unix timestamp of the time when the reminder happens

  // misc exports
  // the format of the timer
  export let format: string =
    dayjs(time).date() === dayjs().date() &&
    dayjs(time).month() === dayjs().month()
      ? "hh:mm a"
      : "MM/DD/YYYY, hh:mm a";
  // the delay until the timer is destroyed
  export let timerDestroyDelay: number = 3000;

  // gets the top level node for this
  // component, allows for it to be destroyed.
  let nodeRef: any;
  // checks if the checkbox is checked
  let checked: boolean;
</script>

<!-- get the top-level component -->
<div bind:this={nodeRef}>
  <label for={name} class="flex items-center p-2">
    <input
      type="checkbox"
      class="mx-2 w-6 h-6 rounded-full bg-secondary !focus:outline-none !outline-none accent-accent border-none cursor-pointer"
      bind:checked
      on:click={async () => {
        // wait 3 seconds if user doesn't want to destroy this
        await new Promise((r) => setTimeout(r, timerDestroyDelay));
        // if the reminder is checked by this
        // time, destroy it.
        if (checked) {
          // tell the app that this reminder is gone
          remove(id);
        }
        // TODO: reset timer if checked in the
        // middle of waiting for timer to go off
      }}
    />
    <p style={(checked ? "opacity: 0.65;" : "") + red} class="m-0">
      <span class="text-4xl">
        {name}
      </span>
      <span class="text-xl opacity-80">
        {dayjs(time).format(format)}
      </span>
    </p>
  </label>
</div>
