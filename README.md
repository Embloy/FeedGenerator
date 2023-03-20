#### <div style="text-align:right">E-FG-0001 </div>

# [FG-API](https://embloy-fg-api.onrender.com)

> __NOTE__: _If you are looking for concrete HTTPs-Request examples instead, go to_
___[.http](requests/feed.http)___

## FUNCTIONALITY

This repository contains the code for the [FeedGenerator-API](https://embloy-fg-api.onrender.com) that is used
by [embloy.com](embloy.com) to generate a feed
from a set of
jobs that matches with the user's preferences.

This API supports [TLS (Transport Layer Security) Encryption](https://actix.rs/docs/server#tls--https):
> The ***cert.pem*** file contains the TLS certificate that is
> issued to your domain by a Certificate Authority (CA).
> This certificate is used to authenticate
> your server to clients, proving that the server is indeed the server that the client is trying to
> communicate with.
> The certificate contains information such as the domain name it's issued for,
> the issuer's information, and the public key.

> The ***key.pem*** file contains the private key that
> corresponds to the public key in the ***cert.pem*** file. This key is used to encrypt the data that is
> being sent between the client and the server. Both files are required for TLS encryption to
> work properly, and should be kept secure as they grant access to sensitive information.

For testing purposes load TLS keys to create a self-signed temporary cert:

```
$ openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
```

---

## ENDPOINTS

### 1. Root

Test if your connection is active.

> <span style="color:lawngreen"> GET </span> /

---

### 2. Auth

Authenticate with Basic Authentication to check if your credentials are valid.

> <span style="color:lawngreen"> GET </span> /auth

####

<details>
  <summary>Data parameters</summary>

1. **username** _<span style="color:crimson">REQUIRED </span>_
2. **password** _<span style="color:crimson">REQUIRED </span>_

</details>

<details>
  <summary>Response body</summary>

**200: Ok**

```
{
"Authenticated"
}
```

**401: Unauthorized**

```
{
"Unauthorized"
}
```

</details>

---

### 3. Feed

Request to load a feed from a "slice" to match the user's preferences.

> <span style="color:lawngreen"> POST </span> /feed


<details>
  <summary>Data parameters</summary>

1. **username** _<span style="color:crimson">REQUIRED </span>_
2. **password** _<span style="color:crimson">REQUIRED </span>_

</details>


<details>
  <summary>Request body</summary>

###### Request body: `web::Json<FeedRequest>`

```
{
  "pref": {
    "job_type": "Software Engineer",
    "salary_range": [
      80000.0,
      120000.0
    ],
    "spontaneity": [
      "2023-03-17T12:00:00Z",
      "2023-03-17T12:00:00Z"
    ]
  },
  "slice": [
    {
      "job_id": 1,
      "job_type": "Data Scientist",
      "job_status": 1,
      "status": "open",
      "user_id": 1,
      "duration": 6,
      "code_lang": "Python",
      "title": "Data Scientist",
      "position": "Junior",
      "description": "We're looking for a junior data scientist to help us analyze our data and build predictive models.",
      "key_skills": "Python, statistics, machine learning",
      "salary": 60000.0,
      "currency": "USD",
      "image_url": "https://example.com/image.jpg",
      "start_slot": "2023-04-01T00:00:00Z",
      "longitude": -122.4194,
      "latitude": 37.7749,
      "country_code": "US",
      "postal_code": "94103",
      "city": "San Francisco",
      "address": "123 Main St",
      "view_count": 50,
      "created_at": "2023-03-17T12:00:00Z",
      "updated_at": "2023-03-17T12:00:00Z",
      "applications_count": 5,
      "job_notifications": "email",
      "employer_rating": 2
    },
    {
      "job_id": 6588,
      "job_type": "Farming",
      "job_status": 0,
      "status": "public",
      "user_id": 367,
      "duration": 1958,
      "code_lang": "KG",
      "title": "Mining Planner",
      "position": "Assistant",
      "description": "Rhetoric is the art of ruling the minds of men.",
      "key_skills": "Teamwork",
      "salary": 77851,
      "currency": "Iranian Rial",
      "image_url": "https://picsum.photos/200/300?random=6588",
      "start_slot": "1975-07-21T13:36:42.000Z",
      "longitude": 34425.61,
      "latitude": 91551.04,
      "country_code": "RE",
      "postal_code": "63478-4176",
      "city": "Lake Tatyana",
      "address": "312 Agustin Plain",
      "view_count": 677,
      "created_at": "2023-02-28T02:06:57.681Z",
      "updated_at": "2023-02-28T02:06:57.681Z",
      "applications_count": 0,
      "job_notifications": "true",
      "employer_rating": 5
    }
  ]
}
```

</details>


<details>
  <summary>Response body</summary>


**200: Ok**

```
[
    {
        "job_id": 6588,
        "job_type": "Farming",
        "job_status": 0,
        "status": "public",
        "user_id": 367,
        "duration": 1958,
        "code_lang": "KG",
        "title": "Mining Planner",
        "position": "Assistant",
        "description": "Rhetoric is the art of ruling the minds of men.",
        "key_skills": "Teamwork",
        "salary": 77851,
        "currency": "Iranian Rial",
        "image_url": "https://picsum.photos/200/300?random=6588",
        "start_slot": "1975-07-21T13:36:42.000Z",
        "longitude": 34425.61,
        "latitude": 91551.04,
        "country_code": "RE",
        "postal_code": "63478-4176",
        "city": "Lake Tatyana",
        "address": "312 Agustin Plain",
        "view_count": 677,
        "created_at": "2023-02-28T02:06:57.681Z",
        "updated_at": "2023-02-28T02:06:57.681Z",
        "applications_count": 0,
        "job_notifications": "true"
    }
]
```

####

**400: Bad Request**

```
NOT IMPLEMENTED YET
```

**500: Internal Server Error**

```
NOT IMPLEMENTED YET
```

####

**401: Unauthorized**
</details>

---
© Carlo Bortolan, Jan Hummel

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [@carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [@bortolanoffice@embloy.com](bortolanoffice@embloy.com)
>
> Jan Hummel &nbsp;&middot;&nbsp;
> GitHub [@github4touchdouble](https://github.com/github4touchdouble) &nbsp;&middot;&nbsp;
> contact via [@hummeloffice@embloy.com](hummeloffice@embloy.com)



