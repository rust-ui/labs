const container = document.querySelector(".container");

const createButton = /*html*/ `
<button class="help-btn">
  <svg width="32" height="32" viewBox="0 0 32 32" fill="none">
    <path d="M3 17c0-5.6 4.4-10 10-10s10 4.4 10 10m-18 0v6c0 1.2.8 2 2 2h4v-8h-4m14 0v6c0 1.2-.8 2-2 2h-4v-8h4" 
      stroke="currentColor" 
      stroke-width="2" 
      stroke-linecap="round" 
      stroke-linejoin="round"/>
  </svg>
  <span class="button-label">Need help?</span>
</button>
`;

const createCard = /*html*/ `
<div class="card">
  <div class="card-header">
    <span class="card-label">Need Help?</span>
    <button class="close-btn">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
        <path d="M18 6L6 18M6 6l12 12" stroke-width="2" stroke-linecap="round"/>
      </svg>
    </button>
  </div>
  <div class="card-subtitle">
    We're here to help 24/7. Choose your preferred contact method below.
  </div>
  <div class="grid">
    <div class="grid-item" data-animation-delay="0.2">
      <svg class="icon" viewBox="0 0 24 24" fill="none">
        <path d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
      <span>Raise a ticket</span>
      <span style="color: #71717a; font-weight: normal; margin-left: auto">Get a reply within 24 hours</span>
      <svg class="icon" viewBox="0 0 24 24" fill="none">
        <path d="M9 18l6-6-6-6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
    </div>

    <div class="grid-item" data-animation-delay="0.25">
      <svg class="icon" viewBox="0 0 24 24" fill="none">
        <path d="M21 11.5a8.38 8.38 0 01-.9 3.8 8.5 8.5 0 01-7.6 4.7 8.38 8.38 0 01-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 01-.9-3.8 8.5 8.5 0 014.7-7.6 8.38 8.38 0 013.8-.9h.5a8.48 8.48 0 018 8v.5z" stroke="currentColor" stroke-width="2"/>
      </svg>
      <span>Live Chat</span>
      <span style="color: #71717a; font-weight: normal; margin-left: auto">2-3 minute wait time</span>
      <svg class="icon" viewBox="0 0 24 24" fill="none">
        <path d="M9 18l6-6-6-6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
    </div>

    <div class="grid-item" data-animation-delay="0.35">
      <svg class="icon" viewBox="0 0 24 24" fill="none">
        <path d="M22 16.92v3a2 2 0 01-2.18 2 19.79 19.79 0 01-8.63-3.07 19.5 19.5 0 01-6-6 19.79 19.79 0 01-3.07-8.67A2 2 0 014.11 2h3a2 2 0 012 1.72 12.84 12.84 0 00.7 2.81 2 2 0 01-.45 2.11L8.09 9.91a16 16 0 006 6l1.27-1.27a2 2 0 012.11-.45 12.84 12.84 0 002.81.7A2 2 0 0122 16.92z" stroke="currentColor" stroke-width="2"/>
      </svg>
      <span>Talk to us</span>
      <span style="color: #71717a; font-weight: normal; margin-left: auto">Receive an instant callback</span>
      <svg class="icon" viewBox="0 0 24 24" fill="none">
        <path d="M9 18l6-6-6-6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
    </div>
  </div>
</div>
`;

const ticketForm = /*html*/ `
<div class="card">
  <div class="card-header">
    <button class="back-btn">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
        <path d="M15 18l-6-6 6-6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
      <span>Raise a ticket</span>
    </button>
  </div>
  <p class="card-subtitle">
    We typically reply within 24 hours. Please provide as much
  </p>
  <form>
    <div class="form-group">
      <label>Email</label>
      <input type="email" required>
    </div>
    <div class="form-group">
      <label>Message</label>
      <textarea required></textarea>
    </div>
    <button type="submit" class="submit-btn">Submit</button>
  </form>
</div>
`;

container.addEventListener("click", (e) => {
  const btn = e.target.closest("button");
  const gridItem = e.target.closest(".grid-item");
  if (!btn && !gridItem) return;

  if (btn?.classList.contains("help-btn")) {
    document.startViewTransition?.({
      update: () => {
        container.innerHTML = createCard;
      },
      types: ["to-card"],
    });
  }

  if (btn?.classList.contains("close-btn")) {
    document.startViewTransition?.({
      update: () => {
        container.innerHTML = createButton;
      },
      types: ["back-to-button"],
    });
  }

  if (btn?.classList.contains("back-btn")) {
    document.startViewTransition?.({
      update: () => {
        container.innerHTML = createCard;
      },
      types: ["back-to-card"],
    });
  }

  if (gridItem?.querySelector("span")?.textContent === "Raise a ticket") {
    gridItem.classList.add("active");

    document.startViewTransition?.({
      update: () => {
        container.innerHTML = ticketForm;
      },
      types: ["to-ticket-form"],
    });
  }
});
