require 'json'
require 'httparty'

class UserService
  def initialize
    @users = {}
  end

  def create_user(name)
    id = @users.length + 1
    user = { id: id, name: name }
    @users[id] = user
    user
  end

  def get_user(id)
    @users[id]
  end

  def delete_user(id)
    @users.delete(id)
  end
end

class AdminService < UserService
  def create_admin(name, role)
    user = create_user(name)
    user[:role] = role
    user
  end
end

def helper_function
  'helper'
end
