import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/v1');
const globalAny:any = global;

describe('Origin Invitations API', function() { 
  describe('Invite bobo to xmen', function() {
    it('refuses invitations from non-members', function(done) {
      request.post('/depot/origins/xmen/users/bobo/invitations')
        .set('Authorization', globalAny.bobo_bearer)
        .expect(403)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the invitation', function(done) {
      request.post('/depot/origins/xmen/users/bobo/invitations')
        .set('Authorization', globalAny.logan_bearer)
        .expect(201)
        .end(function(err, res) {
          expect(res.body.account_id).to.equal(globalAny.session_bobo.id);
          expect(res.body.origin_id).to.equal(globalAny.origin_xmen.id);
          expect(res.body.owner_id).to.equal(globalAny.session_logan.id);
          globalAny.invite_bobo_to_xmen = res.body;
          done(err);
        });
    });

    it('should wait for the account service to be updated', function(done){
      this.timeout(10000);
      setTimeout(done, 8000);
    });
  });
});

