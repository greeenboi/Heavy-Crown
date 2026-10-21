# Heavy is the Crown..

Load testing http APIs on my homelab to see the best results..


# Container Constraints

We are using the portainer service inside my ubuntu server. We are constraining every container we are load testing to have a max of `2048`mB of RAM and `1` CPU core.

# API Specs to follow

### Functional requirements for load testing

The load test should cover these three API operations:

1. **Create an item**

   Send a `POST` request to `/items/` with a JSON body:

   ```json
   {
     "title": "Load test item",
     "description": "Item created during the load test"
   }
   ```

   The response should have status `201 Created` and include the generated
   item ID:

   ```json
   {
     "id": 1,
     "title": "Load test item",
     "description": "Item created during the load test"
   }
   ```

2. **Read all items**

   Send a `GET` request to `/items/`.

   The response should have status `200 OK` and return an array of items:

   ```json
   [
     {
       "id": 1,
       "title": "Load test item",
       "description": "Item created during the load test"
     }
   ]
   ```

3. **Read one item**

   Send a `GET` request to `/items/{item_id}`, replacing `{item_id}` with a
   valid item ID such as `1`.

   The response should have status `200 OK` and return one item:

   ```json
   {
     "id": 1,
     "title": "Load test item",
     "description": "Item created during the load test"
   }
   ```


| Operation | Method and endpoint | Sample request | Expected response |
| --- | --- | --- | --- |
| Create an item | `POST /items/` | `{"title":"Load test item","description":"Item created during the load test"}` | `201 Created`<br>`{"id":1,"title":"Load test item","description":"Item created during the load test"}` |
| Read all items | `GET /items/` | None | `200 OK`<br>`[{"id":1,"title":"Load test item","description":"Item created during the load test"}]` |
| Read one item | `GET /items/{item_id}`<br>Example: `/items/1` | None | `200 OK`<br>`{"id":1,"title":"Load test item","description":"Item created during the load test"}` |

---

# Performance


---

made with curiosity by @greeenboi