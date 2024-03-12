use rust_web_ui::*;

pub struct Example6;

pub struct Example6Props {}

impl Default for Example6Props {
	fn default() -> Example6Props {
		Example6Props { }
	}
}

impl Widget<'_> for Example6 {
	type Props = Example6Props;

	fn render(mut ctx: Ctx<'_>, props: Example6Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("bg-white"), Style::Padding(32)]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[Style::Flex, NoStyle::Noop("flex-wrap"), Style::JustifyCenter, NoStyle::Noop("gap-x-6"), NoStyle::Noop("gap-y-4")]);
				
				props.child("1", Dynamic).run("span", |props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), Style::Color(Color::Fg(6)), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(67)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-500/10")]);
					
					props.child("0", Label).run(|props| props.set_text("Badge"));
				});
				props.child("3", Dynamic).run("span", |props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), Style::Color(Color::Danger(6)), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Danger(78)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-red-600/10")]);
					
					props.child("0", Label).run(|props| props.set_text("Badge"));
				});
				props.child("5", Dynamic).run("span", |props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-yellow-50"), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-yellow-800"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-yellow-600/20")]);
					
					props.child("0", Label).run(|props| props.set_text("Badge"));
				});
				props.child("7", Dynamic).run("span", |props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), Style::Color(Color::Success(6)), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Success(78)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-green-600/20")]);
					
					props.child("0", Label).run(|props| props.set_text("Badge"));
				});
				props.child("9", Dynamic).run("span", |props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), Style::Color(Color::Notice(6)), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Notice(78)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-blue-700/10")]);
					
					props.child("0", Label).run(|props| props.set_text("Badge"));
				});
				props.child("11", Dynamic).run("span", |props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-indigo-50"), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-700"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-indigo-700/10")]);
					
					props.child("0", Label).run(|props| props.set_text("Badge"));
				});
				props.child("13", Dynamic).run("span", |props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-purple-50"), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-purple-700"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-purple-700/10")]);
					
					props.child("0", Label).run(|props| props.set_text("Badge"));
				});
				props.child("15", Dynamic).run("span", |props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-pink-50"), Style::PaddingX(8), Style::PaddingY(4), NoStyle::Noop("text-xs"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-pink-700"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-pink-700/10")]);
					
					props.child("0", Label).run(|props| props.set_text("Badge"));
				});
			});
		});
	}
}
