# Serwis aukcyjny

## Funkcje serwisu

- Rejestracja
- Logowanie
- Wystawianie towarów
  - aukcje
  - kup teraz
- Kupowanie / licytacja
- Przeglądanie ofert z filtrowaniem
  - zawiera tekst
  - cena minimalna
  - cena maksymalna
  - ilość przedmiotów 
- Przeglądanie własnych aktualnych akucji i zakupów

## Obsługa towaru (sprzedający/wystawiający)

### kup teraz

- Dodawanie towaru:
  - typ (kupteraz)
  - cena
  - ilość
  - opis
- Modyfikacja
- Usuwanie

### aukcja

- Dodawanie aukcji:
  - typ (aukcja)
  - cena minimalna
  - opis
  - data
- Modyfikacja
  - tylko opis  icena minimalna
- Usuwanie
  - tylko przed końcem aukcji

## API

```json
/registration
POST 
{
    mail:		string,
    password:	string
}

201 - pomyślne utworzenie użytkownika
400 - Bad Request (np. zły email)
409 - konto istnieje
500 - internal server error
503 - server unavaiable (reason in description)

[not POST] 
405 - Method Not Allowed
```



```json
/login
POST 
{
    mail:		string,
    password:	string
}

200 - OK, (JWT token in response)
401 - Unauthorized
404 - user not found
500 - internal server error
503 - server unavaiable (reason in description)

[not POST] 
405 - Method Not Allowed
```



```json
/offers
POST
{
    type*: [auction|buynow],
	description*:	string,
	price*:	float, (cena min. - akcja; cena za sztukę - kup teraz),
	date*:	timestamp, (sekundy, tylko dla aukcji),
	amount*: int (tylko dla kup teraz)
}

201 - added 
	{
    	offer_id: int
	}
400 - Bad Request (niepoprawny JSON, brakujące lub nadmiarowe pola)
401 - Unauthorized (niezalogowany użytkownik)
403 - Forbidden - zalogowany na nieuprawnione konto
500 - internal server error
503 - server unavaiable (reason in description)

GET
allowed filters in url:
- contains - string in offer description
- price_min - minimal price of item (and auctions)
- price_max - maximal price of item (and auctions)
- type - [auction/buynow] - type of offer
- created_by_me: boolean

200 (offers in response - may be empty "[]")
400 - Bad Request (unproper filter parameter in url)
500 - internal server error
503 - server unavaiable (reason in description)
```



```json
/offers/{id}
PATCH
{
    **fields_to_modify...
}

user may modyfi:
- price, amount, description - for buynow type offers
- price, description - for auction type offers

202 - Accpepted
400 - Bad Request or bad field
403 - Unauthorized
404 - Offer with given id not found


DELETE
(no payload)
202 - Accepted
403 - Unauthorized
404 - Offer with given id not found

GET
no payload
200 - details of auction
{	
    type: "auction",
    description: string,
    status: [active / expired],
    last_bid: float,
    customer_id: int,
    expiration_ts: <timestamp>
}
200 - details of buynow
{	
    type: "buynow",
    description: string,
	price: int,        
	amount: int
}
404 - Offer with given id not found
```



```json
/offers/{id}/buy
POST
{
    bid: 	int (for auction)
    amount:	int (for buynow)
}

202 - Accepted
400 - Bad Request (niepoprawny JSON, brakujące lub nadmiarowe pola)
401 - Unauthorized (niezalogowany użytkownik)
409 - Conflict (details below)
	409 - {"minimal_bid": <minimal bid for auction>}
	409 - {"max_amout": <amount of available items>}
	409 - {"status": "expired"}
	409 - {"conflict": "unable to order own items"}
	409 - {"conflict": "you can not bid on the auction you win"}
500 - internal server error
503 - server unavaiable (reason in description)
```


