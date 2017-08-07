import { NewestAngularKataPage } from './app.po';

describe('newest-angular-kata App', () => {
  let page: NewestAngularKataPage;

  beforeEach(() => {
    page = new NewestAngularKataPage();
  });

  it('should display welcome message', () => {
    page.navigateTo();
    expect(page.getParagraphText()).toEqual('Welcome to app!');
  });
});
