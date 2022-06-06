<script lang="ts">
  import dayjs from "dayjs";
  import Reminder from "$lib/components/Reminder.svelte";
</script>

<div class="">
  <h1>mem</h1>
  <Reminder
    {...{
      name: "reminder",
      time: dayjs().second(dayjs().second() + 5),
      timerDestroyDelay: 1000,
    }}
  />
</div>
