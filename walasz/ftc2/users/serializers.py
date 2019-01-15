from django.contrib.auth import get_user_model
from django.contrib.auth.password_validation import validate_password
from django.utils.translation import ugettext_lazy as _
from rest_framework import serializers
from allauth.utils import email_address_exists
from allauth.account.utils import setup_user_email
from allauth.account import app_settings as allauth_settings

# Get the User model
UserModel = get_user_model()


class UserDetailsSerializer(serializers.ModelSerializer):
    """
    User model w/o password
    """

    class Meta:
        model = UserModel
        fields = ('pk', 'email')
        # fields = ('pk', 'email', 'name')
        read_only_fields = ('email',)


class RegisterSerializer(serializers.Serializer):
    email = serializers.EmailField(required=True)
    # name = serializers.CharField()
    password = serializers.CharField(write_only=True)
    # password2 = serializers.CharField(write_only=True)

    # def validate_name(self, name):
    #     return name

    def validate_email(self, email):
        if email and email_address_exists(email):
            raise serializers.ValidationError(
                _("A user is already registered with this e-mail address."))
        return email

    def validate_password(self, password):
        min_length = allauth_settings.PASSWORD_MIN_LENGTH
        if min_length and len(password) < min_length:
            raise serializers.ValidationError(_("Password must be a minimum of {0} "
                                                "characters.").format(min_length))
        validate_password(password)
        return password

    def validate(self, data):
        # if data['password1'] != data['password2']:
        #     raise serializers.ValidationError(_("The two password fields didn't match."))
        return data

    def get_cleaned_data(self):
        return {
            # 'name': self.validated_data.get('name', ''),
            # 'password': self.validated_data.get('password1', ''),
            'password': self.validated_data.get('password', ''),
            'email': self.validated_data.get('email', '')
        }

    def save(self, request):
        cleaned_data = self.get_cleaned_data()
        user = UserModel.objects.create_user(**cleaned_data)
        setup_user_email(request, user, [])
        return user
