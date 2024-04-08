#### <div style="text-align:right">E-FG-0001 </div>

# FeedGenerator

**Feed algorithm used to create custom job feeds (similar to Instagram) for users based on their preferences and a batch of jobs.**

> [!IMPORTANT]
> As of September 14, 2023, this project has been put on hold and will probably not be worked on in the near future.

> [!NOTE]
> This project is unrelated to the current [embloy.com](https://embloy.com) service and consists of deprecated code from a prior project.

> [!NOTE]
> If you are looking for concrete HTTPs-Request examples, go to [requests/feed.http](requests/feed.http).

## FUNCTIONALITY

This repository contains the code for the [FeedGenerator-API](https://embloy-fg-api.onrender.com) that was used for an early test version
by [embloy.com](https://embloy.com) to generate a feed from a set of jobs that matches with the user's preferences.

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

### 2. Feed

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
        "job_types": {
            "1": 15,
            "2": 5,
            "3": 1,
            "5": 15
        },
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
            "job_id": 4024,
            "job_type": "Marketing",
            "job_type_value": 12,
            "job_status": 0,
            "status": "public",
            "user_id": 9,
            "duration": 0,
            "code_lang": "RE",
            "title": "Administration Designer",
            "position": "Analyst",
            "description": {
            "id": 67,
            "name": "description",
            "body": "<div>This is a great job!</div>",
            "record_type": "Job",
            "record_id": 67,
            "created_at": "2023-09-12T19:45:54.684Z",
            "updated_at": "2023-09-12T19:45:54.684Z"
            },
            "key_skills": "Proactive",
            "salary": 60000.0,
            "currency": "Vatu",
            "image_url": "https://picsum.photos/200/300?random=0",
            "start_slot": "2023-04-19T15:39:19.780Z",
            "longitude": 98011.26,
            "latitude": 58804.27,
            "country_code": "TL",
            "postal_code": "07683-5317",
            "city": "Joanland",
            "address": "31836 Bartoletti Points",
            "view_count": 500,
            "created_at": "2023-04-15T15:39:21.151Z",
            "updated_at": "2023-04-15T15:39:21.151Z",
            "applications_count": 100,
            "employer_rating": 4,
            "job_notifications": "1",
            "boost": 48,
            "job_value": null,
            "euro_salary": 0,
            "relevance_score": 0.0,
            "cv_required": true,
            "allowed_cv_format": [".pdf", ".xml", ".txt", ".docx"]
        },
        {
            "job_id": 4025,
            "job_type": "Marketing",
            "job_type_value": 12,
            "job_status": 0,
            "status": "public",
            "user_id": 9,
            "duration": 0,
            "code_lang": "RE",
            "title": "Administration Designer",
            "position": "Analyst",
            "description": {
            "id": 67,
            "name": "description",
            "body": "<div>This is a great job!</div>",
            "record_type": "Job",
            "record_id": 67,
            "created_at": "2023-09-12T19:45:54.684Z",
            "updated_at": "2023-09-12T19:45:54.684Z"
            },
            "key_skills": "Proactive",
            "salary": 60000.0,
            "currency": "Vatu",
            "image_url": "https://picsum.photos/200/300?random=0",
            "start_slot": "2023-04-19T15:39:19.780Z",
            "longitude": 98011.26,
            "latitude": 58804.27,
            "country_code": "TL",
            "postal_code": "07683-5317",
            "city": "Joanland",
            "address": "31836 Bartoletti Points",
            "view_count": 500,
            "created_at": "2023-04-15T15:39:21.151Z",
            "updated_at": "2023-04-15T15:39:21.151Z",
            "applications_count": 100,
            "employer_rating": 4,
            "job_notifications": "1",
            "boost": 48,
            "job_value": null,
            "euro_salary": 0,
            "relevance_score": 0.0,
            "cv_required": true,
            "allowed_cv_format": [".pdf", ".xml", ".txt", ".docx"]
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
        "job_id": 4024,
        "job_type": "Marketing",
        "job_type_value": 12,
        "job_status": 0,
        "status": "public",
        "user_id": 9,
        "duration": 0,
        "code_lang": "RE",
        "title": "Administration Designer",
        "position": "Analyst",
        "description": {
        "id": 67,
        "name": "description",
        "body": "<div>This is a great job!</div>",
        "record_type": "Job",
        "record_id": 67,
        "created_at": "2023-09-12T19:45:54.684Z",
        "updated_at": "2023-09-12T19:45:54.684Z"
        },
        "key_skills": "Proactive",
        "salary": 60000.0,
        "currency": "Vatu",
        "image_url": "https://picsum.photos/200/300?random=0",
        "start_slot": "2023-04-19T15:39:19.780Z",
        "longitude": 98011.26,
        "latitude": 58804.27,
        "country_code": "TL",
        "postal_code": "07683-5317",
        "city": "Joanland",
        "address": "31836 Bartoletti Points",
        "view_count": 500,
        "created_at": "2023-04-15T15:39:21.151Z",
        "updated_at": "2023-04-15T15:39:21.151Z",
        "applications_count": 100,
        "employer_rating": 4,
        "job_notifications": "1",
        "boost": 48,
        "job_value": null,
        "euro_salary": 0,
        "relevance_score": 0.0,
        "cv_required": true,
        "allowed_cv_format": [".pdf", ".xml", ".txt", ".docx"]
    }
]
```

####

**400: Bad Request**

```
NOT IMPLEMENTED YET
```

**401: Unauthorized**

**500: Internal Server Error**

```
NOT IMPLEMENTED YET
```

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



