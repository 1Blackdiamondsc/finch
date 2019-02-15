<div style="text-align:center; margin: 50px 0">
  <img src="docs/finch-logo.png" width="200" />
</div>

## What's Finch

Finch is an open source cryptocurrency payment processor that puts its focuses on easy integration and flexibility.

<div style="text-align:center; margin: 50px 0">
    <img src="https://finch.ams3.cdn.digitaloceanspaces.com/branding/finch-modal.gif" width="600" />
</div>

**Note**: Finch is currently in beta form and may change significantly before version 1.0 is released.

## Demo

Try [public demo](https://app.finchtech.io) of Finch and its Management Console.

## Installation

We support two methods of installing and running your own Finch server. Our recommended approach is to use Docker, but if that’s not a supported environment, you may also setup a Rust environment.

- [Via Docker](https://docs.finchtech.io/docs/installation/installation_with_docker)
- [Via Rust](https://docs.finchtech.io/docs/installation/installation_with_rust)

## Integration with Your Services

Since Finch communicates directly with the client-side of integrated services', our front-end SDK can handle almost everything needed for the integration. We currently provide [JavaScript SDK](https://github.com/finch-tech/finch-sdk-javascript) which allows you to start accepting cryptocurrencies with a block of code;

```js
<script>
  window.onload = function() {
    let finchCheckout = new FinchCheckout({
      apiUrl: "https://api.finchtech.io",
      apiKey: "5tsdghD/RusjgbskoisRrgw==",
      currencies: ["btc", "eth"],
      fiat: "usd",
      price: "1.2",
      identifier: "hello@example.com",
      button: document.getElementById("pay-with-crypto"),
      onSuccess: function(voucher) {
        // Here you can get signed payment voucher in the form of JSON Web Token.
        // What you need to do on your service's backend is just to verify
        // this voucher using JWT decode library of your choice.
        console.log("Successfully completed the payment.", voucher);
      }
    });
    finchCheckout.init();
  };
</script>
```

After users successfully complete the payment, `onSuccess` callback will be called, and you'll receive a payment voucher (JSON Web Token) as a parameter. Send the voucher to your service's backend so that you can decode and verify it.
Please refer to the [official documentation](https://docs.finchtech.io/docs/getting_started/payment_verification) for more detailed explanation on our payment voucher.

## Store Management Console

We also provide an open source web-based [store management console](https://github.com/finch-tech/finch-management-console).

## Resources

- [Documentation](https://docs.finchtech.io/docs/home/overview.html)
- [Installation Guide](https://docs.finchtech.io/docs/installation/server)
- [Getting Started Guide](https://docs.finchtech.io/docs/getting_started/overview) (Store Setup and Integration)
- [Payment API Documentation](https://docs.finchtech.io/docs/payment_api/payments/create)
- [Store Management API Documentation](https://docs.finchtech.io/docs/management_api/auth/registration)
