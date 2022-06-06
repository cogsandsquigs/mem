<script lang="ts">
  import dayjs from "dayjs";
  // not needed (yet)
  // import utc from "dayjs/plugin/utc";
  // import timezone from "dayjs/plugin/timezone"; // dependent on utc plugin
  // dayjs.extend(utc);
  // dayjs.extend(timezone);

  export let name: string = "..."; // the name of the reminder
  export let time: number | dayjs.Dayjs | Date = dayjs()
    .hour(dayjs().hour() + 1)
    .minute(Math.round(dayjs().minute() / 5) * 5); // the unix timestamp of the time when the reminder happens
  export let format: string =
    dayjs(time).date() === dayjs().date() &&
    dayjs(time).month() === dayjs().month()
      ? "hh:mm a"
      : "MM/DD/YYYY, hh:mm a";
</script>

<div>
  <label for={name} class="flex items-center p-2">
    <input
      type="checkbox"
      class="mx-2 w-6 h-6 rounded-full bg-secondary !focus:outline-none !outline-none accent-accent border-none cursor-pointer"
    />
    <p class="m-0">
      <span class="text-4xl">
        {name}
      </span>
      <span class="text-xl opacity-80">
        {dayjs(time).format(format)}
      </span>
    </p>
  </label>
</div>

<style>
  input[type="checkbox"] {
    accent-color: red;
  }
</style>
