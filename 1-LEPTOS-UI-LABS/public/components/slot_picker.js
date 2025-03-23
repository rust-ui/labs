const days = document.querySelectorAll('.day');
    
days.forEach((day, dayIndex) => {
  const timeSlots = day.querySelector('.time-slots');
  const addBtn = day.querySelector('.add-btn');
  const switchBtn = day.querySelector('input[switch]');

  function createTimeRow(dayIndex, slotIndex) {
    function getRandomTime(start = 0, end = 24) {
      const hour = Math.floor(Math.random() * (end - start) + start);
      const minute = Math.floor(Math.random() * 60);
      return `${hour.toString().padStart(2, '0')}:${minute.toString().padStart(2, '0')}`;
    }

    const fromTime = getRandomTime(6, 18); 
    const toTime = getRandomTime(
      parseInt(fromTime.split(':')[0]) + 1, 
      24
    );

    const row = document.createElement('div');
    row.className = 'time-row';
    row.style.viewTransitionName = `row-${dayIndex}-${slotIndex}`;

    row.innerHTML = `
      <span class="time-label">From</span>
      <input type="time" class="time-input" value="${fromTime}">
      <span class="time-label">To</span>
      <input type="time" class="time-input" value="${toTime}">
      <button class="remove-btn">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <path d="M18 6L6 18M6 6l12 12" stroke-width="2" stroke-linecap="round"/>
        </svg>
      </button>
    `;

    row.querySelector('.remove-btn').addEventListener('click', async() => {
      row.classList.add('remove');

      const transition = document.startViewTransition({
        update: () => {
          row.remove();

        },
        types: ['remove']
      });

      await transition.finished;

      timeSlots.querySelectorAll('.time-row').forEach((row, index) => {
        row.style.viewTransitionName = `row-${dayIndex}-${index}`;
      });
    });

    return row;
  }

  addBtn.addEventListener('click', () => {
    document.startViewTransition(() => {
      const slotIndex = timeSlots.querySelectorAll('.time-row').length;
      const newRow = createTimeRow(dayIndex, slotIndex);
      timeSlots.appendChild(newRow);
    });
  });

  switchBtn.addEventListener('change', () => {
    document.startViewTransition(() => {
      if (switchBtn.checked) {
        day.classList.add('expanded');
      } else {
        day.classList.remove('expanded');
      }
    });
  });
})