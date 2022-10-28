# Optional: Deployment

So far the image filter application has been running locally only.
Of course this can now be deployed to Fastly's servers.

_Note: This requires a Fastly account. You can [create an account](https://www.fastly.com/signup/) for free._

✅ You can now deploy this application:

```
fastly compute deploy
```

The first time you run this it will ask you if you want to create a new service.
Follow the instructions, give it a name,
define a domain to use (or use the suggested one).
You don't need to define any backends.

Subsequent runs will deploy your code as a new version.

When finished this will print the full URL of your new service.
A demo deployment is available at:

<https://forcibly-advanced-eft.edgecompute.app/>
