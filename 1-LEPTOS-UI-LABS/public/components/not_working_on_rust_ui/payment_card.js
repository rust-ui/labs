const cards = [
  {
    name: 'Leonel Ngoya',
    number: '5211300256188330',
    expiry: '12/22',
    cvv: '123',
    logo: 'https://payment-method-lndev.vercel.app/visa.png'
  },
  {
    name: 'Leonel Ngoya',
    number: '5211300256183941',
    expiry: '10/27',
    cvv: '123',
    logo: 'https://payment-method-lndev.vercel.app/mastercard.png'
  },
  {
    name: 'Leonel Ngoya',
    number: '5211300256188330',
    expiry: '06/25',
    cvv: '123',
    logo: 'https://payment-method-lndev.vercel.app/visa.png'
  },
];

const buttonContainer = document.querySelector('.button-container');

const showCardItem = (cardItem) => {
  const index = cardItem.dataset.index;
  const newCardLogo = cardItem.querySelector('.card-logo');
  const newCardNumber = cardItem.querySelector('.card-number-input');
  const newCardExpiry = cardItem.querySelector('.card-expiry-input');

  cardItem.innerHTML = `
    <div class="card-item-content">
      <div class="card-info">
        <img src="${newCardLogo.src}" alt="Visa" class="card-logo" style="view-transition-name: card-logo${index};">
        <div>
          <input type="password" value="${newCardNumber.value}" class="card-number" style="view-transition-name: card-number${index};">
          <input type="text" value="${newCardExpiry.value}" class="card-expiry" style="view-transition-name: card-expiry${index};">
        </div>
      </div>
      <button class="edit-btn" type="button" style="view-transition-name: edit-btn${index};">Edit</button>
    </div>
  `;
};

const showCardList = (cardList) => {
  cardList.innerHTML = cards.map((card, index) => `
      <div class="card-item" data-index="${index + 1}" style="view-transition-name: card-item-edit${index + 1};">
        <div class="card-item-content">
          <div class="card-info">
            <img 
              src="${card.logo}" 
              alt="Visa" 
              class="card-logo" 
              style="view-transition-name: card-logo${index + 1};">
            <div>
              <input 
                type="password" 
                value="${card.number}" 
                class="card-number"
                style="view-transition-name: card-number${index + 1};">
              <input 
                type="text" 
                value="${card.expiry}" 
                class="card-expiry"
                style="view-transition-name: card-expiry${index + 1};">
            </div>
          </div>
          <button class="edit-btn" type="button" style="view-transition-name: edit-btn${index + 1};">Edit</button>
        </div>
      </div>
    `).join('');
};

const showEditForm = (cardItem) => {
  const index = cardItem.dataset.index;
  const newCardLogo = cardItem.querySelector('.card-logo');
  const newCardNumber = cardItem.querySelector('.card-number');
  const newCardExpiry = cardItem.querySelector('.card-expiry');

  const editForm = /*html*/`
    <form class="edit-form">
      <label>Name on card</label>
      <div class="form-group">
        <input type="text" value="Leonel Ngoya">
      </div>
      <label>Card number</label>
      <div class="form-group">
        <img src="${newCardLogo.src}" alt="Visa" class="card-logo" style="view-transition-name: card-logo${index};">
        <input type="text" value="${newCardNumber.value}" class="card-number-input" style="view-transition-name: card-number${index};">
      </div>
      <div class="form-row">
        <div style="flex: 1;">
          <label>Expiry</label>
          <div class="form-group">
            <input type="text" value="${newCardExpiry.value}" class="card-expiry-input" style="view-transition-name: card-expiry${index};">
          </div>
        </div>

        <div style="flex: 1;">
          <label>CVV</label>
          <div class="form-group">
            <input type="password" value="123">
          </div>
        </div>
      </div>
      <button type="button" class="save-btn">
        Save
      </button>
    </form>
  `;

  cardItem.innerHTML = editForm;
};

const addCardForm = (item) => /*html*/`
  <form class="add-form" id="add-form">
    <div class="add-form-header">
      <span>Credit card</span>
    </div>

    <div class="add-form-footer">
      <div>
        <label>Leonel Ngoya</label>
        <input type="text" value="${item.number}" class="card-number-input">
      </div>
      <div>
        <label class="card-expiry">${item.expiry}</label>
        <input type="password" value="${item.cvv}">
      </div>
      <div>
        <img src="${item.logo}" alt="Visa" class="card-logo">
      </div>
    </div>
  </form>
`;

const addSaveCardButton = (buttonContainer) => {
  buttonContainer.innerHTML = /*html*/`
    <button class="save-card-btn in">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
        <path d="M20 6L9 17L4 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <span>Save</span>
    </button>
  `;
}

const addAddCardButton = (buttonContainer) => {
  buttonContainer.innerHTML = /*html*/` 
    <button class="add-card-btn in">
      <svg width="16" height="16" viewBox="0 0 16 16">
        <path d="M8 2v12M2 8h12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <span>Add card</span>
    </button>
  `;
}

const paymentContainer = document.querySelector('.payment-container');
const cardList = document.querySelector('.card-list');

paymentContainer.addEventListener('click', async(e) => {
  const path = e.composedPath();
  const btn = path.find(item => item.tagName && item.tagName.toLowerCase() === 'button');

  if(!btn) return;

  if (btn.classList.contains('edit-btn')) {
    const cardItems = [...cardList.querySelectorAll('.card-item')];
    const currentEdit = cardItems.find(item => item.classList.contains('edit'));
    const currentCardItem = btn.closest('.card-item');

    if(currentEdit) {
      const currentCardItemContent = currentCardItem.querySelector('.card-item-content');
      const editForm = currentEdit.querySelector('.edit-form');

      document.startViewTransition({
        update: () => {
          currentEdit.classList.remove('edit');
          currentCardItem.classList.add('edit');

          showEditForm(currentCardItem);
          showCardItem(currentEdit);
        },
        types: ['swap-edit']
      });

      return;
    }
    else {
      currentCardItem.classList.add('edit');
    }

    document.startViewTransition({
      update: () => showEditForm(currentCardItem),
      types: ['regular']
    });
  }

  if(btn.classList.contains('save-btn')) {
    const cardItem = btn.closest('.card-item');
    const transition = document.startViewTransition({
      update: () => showCardItem(cardItem),
      types: ['regular']
    });

    await transition.finished;

    cardItem.classList.remove('edit');
  }

  if(btn.classList.contains('add-card-btn')) {
    const cardItem = cardList.querySelector('.card-item');
    const index = cards.length % 2;

    btn.classList.add('clicked');

    let transition = document.startViewTransition(() => {
      cardList.innerHTML = addCardForm(cards[index]);

      cardList.insertAdjacentHTML('afterend', '<button type="button" class="cancel-btn" form="add-form">Cancel</button>');
    });

    await transition.finished;

    transition = document.startViewTransition(() => addSaveCardButton(buttonContainer));

    await transition.finished;
  }

  if(btn.classList.contains('save-card-btn')) {
    const cancelBtn = document.querySelector('.cancel-btn');

    const index = cards.length % 2;
    cards.push(cards[index]);

    btn.classList.add('clicked');

    const transition = document.startViewTransition({
      update: () => {
        cancelBtn.remove();
        showCardList(cardList);
        addAddCardButton(buttonContainer);
      },
      types: ['add']
    });

    await transition.finished;

    showCardList(cardList);
  }

  if(btn.classList.contains('cancel-btn')) {
    document.startViewTransition(() => {
      btn.remove();
      showCardList(cardList);
      addAddCardButton(buttonContainer);
    });
  }
});