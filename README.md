#### <div style="text-align:right">P-XJH-0006 </div>

####

# FG-API documentation

> __NOTE__: _If you are looking for concrete HTTPs-Request examples instead, go to_
___[.http](requests/feed.http)___

---

### 0. Root

Test if your connection is active.

> <span style="color:lawngreen"> GET </span> /

---

### 1. Auth

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

####

**401: Unauthorized**

```
{
"Unauthorized"
}
```

</details>

---

### 2. Feed

Request to load a feed from a "slice" to match the user's preferences.

> <span style="color:lawngreen"> POST </span> /feed

####

<details>
  <summary>Request body</summary>

###### Request body: `web::Json<Vec<Job>>`

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
