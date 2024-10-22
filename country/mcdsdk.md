# McDonald's Australia (Maccas of Australia) & USA API documentation

Before you access McDonald's Australia & USA server...

The first thing, you need a security token which requires to get JWT Token by Basic authentication with base64 encoded result.

I can verify that both countries are using MCDSDK.

Australia connection: `https://ap-prod.api.mcd.com/`

USA connection: `https://us-prod.api.mcd.com/`

Main menu of Australia: `https://www.mcdonalds.com/graphql/execute.json/gma/au-en-marketing-android`

Alternative menu of Australia: `https://www.mcdonalds.com/graphql/execute.json/gma/au-en-hero-android`

Main menu of USA: `https://www.mcdonalds.com/graphql/execute.json/gma/us-en-marketing-android`

Alternative menu of Australia: `https://www.mcdonalds.com/graphql/execute.json/gma/au-en-hero-android`

Home menu of Australia: `https://gmabannerscms-prod.s3.amazonaws.com/gma/Home-Menu.json`

## Security auth token before access unlimited (required)

`v1/security/auth/token`

Method: **POST**

This requires authentication argument by base64 encoded like this:

```
Basic OGNHY2tSNXdQZ1FuRkJjOWRlVmhKMnZUOTRXaE1CUkw6WW00clZ5cXBxTnBDcG1yZFBHSmF0UnJCTUhoSmdyMjY=
```

Decoded result:

```
8cGckR5wPgQnFBc9deVhJ2vT94WhMBRL:Ym4rVyqpqNpCpmrdPGJatRrBMHhJgr26
```

The first half is mcd-clientid and last half is mcd-clientsecret.

```
Ym4rVyqpqNpCpmrdPGJatRrBMHhJgr26
```

Codes:

`20000: The call was successful`

You will get JWT token authentication with expiration 15 minutes.

`41600: Access Denied for Current Region.`

This means you can't access outside servers like USA when connected in Australia with located IP address.

## Identify email

`exp/v1/customer/identity/email`

Method: **POST**

Example payload:

```
{
    "customerIdentifier": "example@example.com",
    "deviceId": "0abcdefghijklmno",
    "registrationType": "traditional"
}
```

`customerIdentifier`: A email address of string

`deviceId`: Random Android Device ID string of 16-bit

`registrationType`: A registration type

Codes:

`20000: User exist for the given registration type.`

When code returns successfully, this will send link with base64 encoded of JWT token to email-address associated with existing client account.

`41447: Account does not exists.`

This means the account is not registered of email address to McDonald's.

## Registering an account as new customer

`exp/v2/customer/registration`

Example payload with huge:

```
{
    "address": {
        "country": "AU",
        "zipCode": "2600"
    },
    "audit": {
        "registrationChannel": "M"
    },
    "credentials": {
        "loginUsername": "example@example.com",
        "sendMagicLink": true,
        "type": "email"
    },
    "device": {
        "deviceId": "0abcdefghijklmno",
        "deviceIdType": "AndroidId",
        "isActive": "Y",
        "os": "android",
        "osVersion": "14",
        "timezone": "Australia/Sydney"
    },
    "emailAddress": "example@example.com",
    "firstName": "Example",
    "lastName": "Too",
    "optInForMarketing": false,
    "policies": {
        "acceptancePolicies": {
            "1": true,
            "4": true
        }
    },
    "preferences": [
        {
            "details": {
                "Email": "en-AU",
                "MobileApp": "en-AU",
                "legacyId": "1"
            },
            "preferenceId": 1
        },
        {
            "details": {
                "Email": "N",
                "MobileApp": "N",
                "legacyId": "2"
            },
            "preferenceId": 2
        },
        {
            "details": {
                "Email": "False",
                "MobileApp": "True",
                "legacyId": "3"
            },
            "preferenceId": 3
        },
        {
            "details": {
                "Email": "123456",
                "MobileApp": "123456",
                "legacyId": "4"
            },
            "preferenceId": 4
        },
        {
            "details": {
                "Email": "False",
                "MobileApp": "True",
                "legacyId": "6"
            },
            "preferenceId": 6
        },
        {
            "details": {
                "Email": "False",
                "MobileApp": "True",
                "legacyId": "7"
            },
            "preferenceId": 7
        },
        {
            "details": {
                "Email": "False",
                "MobileApp": "True",
                "legacyId": "8"
            },
            "preferenceId": 8
        },
        {
            "details": {
                "Email": "False",
                "MobileApp": "True",
                "legacyId": "9"
            },
            "preferenceId": 9
        },
        {
            "details": {
                "Email": "False",
                "MobileApp": "True",
                "legacyId": "10"
            },
            "preferenceId": 10
        },
        {
            "details": {
                "Email": "[1,2,3]",
                "MobileApp": "[4,5]",
                "legacyId": "18"
            },
            "preferenceId": 11
        },
        {
            "details": {
                "enabled": "Y"
            },
            "preferenceId": 12
        },
        {
            "details": {
                "enabled": "Y"
            },
            "preferenceId": 13
        },
        {
            "details": {
                "enabled": "Y"
            },
            "preferenceId": 14
        },
        {
            "details": {
                "enabled": "Y"
            },
            "preferenceId": 15
        },
        {
            "details": {
                "enabled": "Y"
            },
            "preferenceId": 16
        },
        {
            "details": {
                "enabled": "Y"
            },
            "preferenceId": 17
        },
        {
            "details": {
                "enabled": "Y"
            },
            "preferenceId": 18
        },
        {
            "details": {
                "enabled": "N"
            },
            "preferenceId": 19
        },
        {
            "details": {
                "enabled": "N"
            },
            "preferenceId": 20
        },
        {
            "details": {
                "enabled": "N"
            },
            "preferenceId": 21
        },
        {
            "details": {
                "enabled": "N"
            },
            "preferenceId": 22
        }
    ],
    "subscriptions": [
        {
            "optInStatus": "Y",
            "subscriptionId": "1"
        },
        {
            "optInStatus": "Y",
            "subscriptionId": "2"
        },
        {
            "optInStatus": "Y",
            "subscriptionId": "3"
        },
        {
            "optInStatus": "Y",
            "subscriptionId": "4"
        },
        {
            "optInStatus": "Y",
            "subscriptionId": "5"
        },
        {
            "optInStatus": "Y",
            "subscriptionId": "7"
        },
        {
            "optInStatus": "N",
            "subscriptionId": "10"
        },
        {
            "optInStatus": "N",
            "subscriptionId": "11"
        },
        {
            "optInStatus": "Y",
            "subscriptionId": "24"
        },
        {
            "optInStatus": "Y",
            "subscriptionId": "25"
        }
    ]
}
```

`20000: The account registration was successful.`

After registering McDonald's account by accepting Terms & Conditions (T&C) and Privacy Policy only, but optionally for news updates like newspapers is not necessary needed.

Also this will send an email login link to your new registered account.

Hidden responses are `accessToken` and `refreshToken`.

`41496: The email address provided is invalid.`

This could sometimes throw an error when email address provided is invalid for some subdomain emails or unrecognized.

Try with different email address, if unsure.

## Troubleshooting:

To solve error code of 41600, for example:

When you want to connect McDonald's USA server in Australia to avoid an issue, you need a USA VPN internet connection access to bypass through.

Coming soon...