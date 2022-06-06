<script lang="ts">
  import dayjs from "dayjs";
  import Reminder from "$components/Reminder.svelte";
</script>

<h1>mem</h1>
<div>
<Reminder
  {...{
    name: "reminder",
    time: dayjs().second(dayjs().second() + 5),
    timerDestroyDelay: 0,
  }}
/>
</div>
