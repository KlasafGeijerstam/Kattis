import requests
from getpass import getpass
from bs4 import BeautifulSoup

if __name__ == '__main__':
    with requests.Session() as s:
        login_page = s.get('https://open.kattis.com/login/email')
        soup = BeautifulSoup(login_page.text, 'html.parser')
        csrf = soup.select('input[name="csrf_token"]')[0]['value']
        email = input('Email: ')
        data = {'csrf_token': csrf, 'password': getpass('Pw: '), 'submit': 'Submit', 'user': email}
        p = s.post('https://open.kattis.com/login/email', data=data)

        if p.status_code != 200:
            print('Login failed')
            exit(1)
        print('Login successfull!')
        with open('data') as f:
            for k in f:
                r = s.get('https://open.kattis.com/submissions/{}/source'.format(k.strip()))
                print('Downloaded {}'.format(k.strip()))
                with open(k.strip() + '.zip', 'wb') as su:
                    su.write(r.content)
