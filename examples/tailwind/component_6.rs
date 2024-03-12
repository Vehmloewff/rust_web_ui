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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("bg-white"), Style::Padding(Size::Exact(32))]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Flex, Style::Noop("flex-wrap"), Style::JustifyCenter, Style::Noop("gap-x-6"), Style::Noop("gap-y-4")]);
				
				props.child("1", Dynamic).run("span", |mut props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Color(Color::Fg(6)), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(67)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-500/10")]);
					
					props.child("0", Label).run(|props| props.text("Badge"));
				});
				props.child("3", Dynamic).run("span", |mut props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Color(Color::Danger(6)), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::TextColor(Color::Danger(78)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-red-600/10")]);
					
					props.child("0", Label).run(|props| props.text("Badge"));
				});
				props.child("5", Dynamic).run("span", |mut props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Noop("bg-yellow-50"), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::Noop("text-yellow-800"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-yellow-600/20")]);
					
					props.child("0", Label).run(|props| props.text("Badge"));
				});
				props.child("7", Dynamic).run("span", |mut props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Color(Color::Success(6)), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::TextColor(Color::Success(78)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-green-600/20")]);
					
					props.child("0", Label).run(|props| props.text("Badge"));
				});
				props.child("9", Dynamic).run("span", |mut props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Color(Color::Notice(6)), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::TextColor(Color::Notice(78)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-blue-700/10")]);
					
					props.child("0", Label).run(|props| props.text("Badge"));
				});
				props.child("11", Dynamic).run("span", |mut props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Noop("bg-indigo-50"), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::Noop("text-indigo-700"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-indigo-700/10")]);
					
					props.child("0", Label).run(|props| props.text("Badge"));
				});
				props.child("13", Dynamic).run("span", |mut props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Noop("bg-purple-50"), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::Noop("text-purple-700"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-purple-700/10")]);
					
					props.child("0", Label).run(|props| props.text("Badge"));
				});
				props.child("15", Dynamic).run("span", |mut props| {
					props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Noop("bg-pink-50"), Style::PaddingX(Size::Exact(8)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-xs"), Style::Noop("font-medium"), Style::Noop("text-pink-700"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-pink-700/10")]);
					
					props.child("0", Label).run(|props| props.text("Badge"));
				});
			});
		});
	}
}
