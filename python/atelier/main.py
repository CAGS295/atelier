
import atelier as atelier

r_ob = atelier.atelier.Orderbook()

print(r_ob.orderbook_id)
print(r_ob.orderbook_ts)
print(r_ob.levels)

ob = r_ob.new_random()
print(ob.levels)
