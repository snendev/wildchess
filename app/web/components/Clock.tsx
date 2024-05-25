interface ClockProps {
  time: string
}

export default function Clock({ time }: ClockProps) {
  return (
    <div class="w-[70px] text-xl text-center border-2 border-black bg-[#fffdf7]">
      {time}
    </div>
  );
}
