// Get context
const ctx = document.getElementById('orderReportChart').getContext('2d');

// Data for the chart
const data = {
  labels: ['Sat', 'Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri'],
  datasets: [
    {
      label: 'Delivered',
      data: [800, 500, 1000, 1100, 950, 700, 600],
      type: 'bar',
      backgroundColor: '#22c55e',
      borderWidth: 0,
      borderRadius: 5,
    },
    {
      label: 'Shipping',
      data: [400, 600, 800, 850, 900, 700, 500],
      type: 'line',
      borderColor: '#3b82f6',
      borderWidth: 2,
      tension: 0.4,
      fill: true,
      backgroundColor: 'rgba(59, 130, 246, 0.1)',
    },
    {
      label: 'Cancelled',
      data: [100, 200, 300, 400, 350, 300, 200],
      type: 'line',
      borderColor: '#f43f5e',
      borderWidth: 2,
      pointBackgroundColor: '#f43f5e',
      tension: 0.4,
    },
    {
      label: 'Returned',
      data: [50, 100, 80, 150, 100, 50, 75],
      type: 'line',
      borderColor: '#f97316',
      borderWidth: 2,
      pointBackgroundColor: '#f97316',
      tension: 0.4,
    },
  ],
};

    const config = {
  type: 'bar',
  data,
  options: {
    responsive: true,
    plugins: {
      legend: {
        display: true,
        position: 'top',
      },
    },
    scales: {
      x: {
        grid: { display: false },
      },
      y: {
        beginAtZero: true,
        grid: { drawBorder: false },
      },
    },
  },
};
  new Chart(ctx, config);